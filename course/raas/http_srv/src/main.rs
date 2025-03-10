#![allow(dead_code, unused)]

use async_trait::async_trait;
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::{
    atomic::{AtomicUsize, Ordering::Relaxed},
    Arc,
};
use std::time::Duration;

use axum::body::Body;
use axum::extract::{Request, State};
use axum::http::{method, version, HeaderMap};
use axum::middleware::Next;
use axum::response::IntoResponse;
use axum::{extract::Path, extract::Query, response::Html, routing::get, Router};
use axum::{middleware, Extension, Json};
use opentelemetry::{global, logs::LogError, trace::TraceError, KeyValue};
use opentelemetry_otlp::{ExportConfig, WithExportConfig};
use opentelemetry_sdk::trace as sdktrace;
use opentelemetry_sdk::{
    logs::Config, metrics::MeterProvider, propagation::TraceContextPropagator, runtime, Resource,
};
use reqwest::StatusCode;
use serde::Serialize;
use tokio::join;
use tower_http::compression::CompressionLayer;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;
use tracing::level_filters::LevelFilter;
use tracing::{info, instrument, Level};
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use utoipa::{OpenApi, ToSchema};
use utoipa_redoc::{Redoc, Servable};
use utoipa_swagger_ui::SwaggerUi;

#[derive(Debug)]
struct MyStruct {
    text: String,
}

#[derive(Debug)]
struct Counter {
    cnt: AtomicUsize,
}

#[derive(Debug)]
struct MyState(i32);

fn service_one() -> Router {
    let state = Arc::new(MyState(5));
    Router::new()
        .route("/", get(service_one_handler))
        .with_state(state)
}

async fn service_one_handler(
    Extension(cnt): Extension<Arc<Counter>>,
    State(state): State<Arc<MyState>>,
) -> Html<String> {
    cnt.cnt.fetch_add(1, Relaxed);
    Html(format!("Service {} - {}", cnt.cnt.load(Relaxed), state.0))
}

fn init_tracer(otlp_endpoint: &str) -> Result<sdktrace::Tracer, TraceError> {
    opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_endpoint(otlp_endpoint),
        )
        .with_trace_config(
            sdktrace::config().with_resource(Resource::new(vec![KeyValue::new(
                "service.name",
                "hello_world",
            )])),
        )
        .install_batch(runtime::Tokio)
}

fn init_metrics(otlp_endpoint: &str) -> opentelemetry::metrics::Result<MeterProvider> {
    let export_config = ExportConfig {
        endpoint: otlp_endpoint.to_string(),
        ..ExportConfig::default()
    };
    opentelemetry_otlp::new_pipeline()
        .metrics(runtime::Tokio)
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_export_config(export_config),
        )
        .with_resource(Resource::new(vec![KeyValue::new(
            opentelemetry_semantic_conventions::resource::SERVICE_NAME,
            "hello_world",
        )]))
        .build()
}

fn init_logs(otlp_endpoint: &str) -> Result<opentelemetry_sdk::logs::Logger, LogError> {
    opentelemetry_otlp::new_pipeline()
        .logging()
        .with_log_config(
            Config::default().with_resource(Resource::new(vec![KeyValue::new(
                opentelemetry_semantic_conventions::resource::SERVICE_NAME,
                "hello_world",
            )])),
        )
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_endpoint(otlp_endpoint.to_string()),
        )
        .install_batch(runtime::Tokio)
}

// OpenAPI documentation annotations.
#[derive(OpenApi)]
#[openapi(
    paths(
        swagg_handler,
    ),
    components(
        schemas(HelloWorld)
    ),
    modifiers(),
    tags(
        (name = "Test System", description = "A really simple API")
    )
)]
struct ApiDoc;

#[tokio::main]
async fn main() {
    // init otel telemetry tools; uncomment when you need it
    // global::set_text_map_propagator(TraceContextPropagator::new());

    // let otlp_endpoint = "http://localhost:4317";

    // let tracer = init_tracer(&otlp_endpoint).unwrap();

    // let telemetry_layer = tracing_opentelemetry::layer().with_tracer(tracer);
    // let subscriber = tracing_subscriber::registry()
    //     .with(LevelFilter::from_level(Level::DEBUG))
    //     .with(telemetry_layer);

    // subscriber.try_init();

    // let _meter_provider = init_metrics(&otlp_endpoint);
    // let _log_provider = init_logs(&otlp_endpoint);

    let _ = dotenvy::dotenv();

    server().await;
}

async fn server() {
    let settings_reader = config::Config::builder()
        .add_source(config::File::with_name("cfg").required(false))
        .add_source(config::Environment::with_prefix("APP"))
        .build()
        .unwrap();

    let settings = settings_reader
        .try_deserialize::<HashMap<String, String>>() // the map can be replaced with a struct, provided that the serde::Deserialize is derived
        .unwrap();

    println!("{settings:?}");

    // logging file init
    let file_appender = tracing_appender::rolling::hourly("log", "server.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    // add tracing subscriber
    let subscriber = tracing_subscriber::fmt()
        .compact()
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_target(false)
        .with_span_events(FmtSpan::CLOSE)
        .json()
        .flatten_event(true)
        .with_writer(non_blocking)
        .finish();

    // set the initialised subscriber as default
    tracing::subscriber::set_global_default(subscriber);

    // tracing_subscriber::fmt().event_format(formatting).init();
    info!("iniialising the web server");

    let shared_counter = Arc::new(Counter {
        cnt: AtomicUsize::new(0),
    });

    let shared_text = Arc::new(MyStruct {
        text: "some".to_string(),
    });

    let other_router = Router::new().route("/status2", get(status2));

    let app = Router::new()
        .nest("/1", service_one()) // sub routing
        .route("/", get(handler))
        .route("/book/{id}", get(path_extract))
        .route("/book", get(query_extract))
        .route("/header", get(header_extract))
        .route("/check", get(req_handler))
        .route("/status", get(status))
        .route("/time", get(handler_time))
        .route("/static", get(static_handler))
        .route("/compressed_file", get(compressed_file))
        .route("/request-id", get(header_handler))
        .route("/swag", get(swagg_handler))
        .route_layer(middleware::from_fn(auth))
        .fallback_service(ServeDir::new("web"))
        .layer(Extension(shared_counter))
        .layer(Extension(shared_text))
        .layer(CompressionLayer::new())
        // NOTE: layer positioning order is important!
        // requires RUST_LOG=debug to be set!
        .layer(
            TraceLayer::new_for_http().make_span_with(|req: &Request<Body>| {
                let req_id = uuid::Uuid::new_v4();
                tracing::span!(
                    tracing::Level::INFO,
                    "request",
                    method = tracing::field::display(req.method()),
                    uri = tracing::field::display(req.uri()),
                    request_id = tracing::field::display(req_id),
                    version = tracing::field::debug(req.version()),
                )
            }),
        )
        .merge(SwaggerUi::new("/swagger-ui").url("/doc/api/openapi.json", ApiDoc::openapi()))
        .merge(Redoc::with_url("/redoc", ApiDoc::openapi()))
        .merge(other_router);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    info!("listening on {}", listener.local_addr().unwrap());

    tokio::spawn(request_with_id());

    axum::serve(listener, app).await.unwrap();
}

#[instrument]
async fn handler(
    Extension(cnt): Extension<Arc<Counter>>,
    Extension(text): Extension<Arc<MyStruct>>,
) -> Json<usize> {
    cnt.cnt.fetch_add(1, Relaxed);
    Json(cnt.cnt.load(Relaxed))
}

// path extraction logic
async fn path_extract(Path(id): Path<u32>) -> Html<String> {
    println!("{id}");
    Html(format!("id passed: {id}"))
}

// query extraction logic
async fn query_extract(Query(params): Query<HashMap<String, String>>) -> Html<String> {
    Html(format!("query params passed: {params:#?}"))
}

// header extraction logic
async fn header_extract(headers: HeaderMap) -> Html<String> {
    Html(format!("headers passed: {headers:#?}"))
}

async fn req_handler() -> Html<String> {
    println!("sending a GET request");
    let cur_cnt = reqwest::get("http://127.0.0.1/")
        .await
        .unwrap()
        .json::<i32>()
        .await
        .unwrap();
    Html(format!("<h1>Remote counter: {cur_cnt}</h1>"))
}

async fn status() -> Result<impl IntoResponse, reqwest::StatusCode> {
    if true {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }
    Ok(Json(42))
}

async fn status2() -> Result<impl IntoResponse, reqwest::StatusCode> {
    if true {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }
    Ok(Json(142))
}

async fn handler_time() -> Result<impl IntoResponse, (reqwest::StatusCode, String)> {
    let start = std::time::SystemTime::now();

    let secs = start
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "bad clock".to_string()))?
        .as_secs()
        % 3;

    let devided = 100u64
        .checked_div(secs)
        .ok_or((StatusCode::INTERNAL_SERVER_ERROR, "div by zero".to_string()))?;

    Ok(Json(devided))
}

async fn static_handler() -> Result<impl IntoResponse, StatusCode> {
    Ok(Html("<h1>static handler</h1>"))
}

async fn header_handler(Extension(auth): Extension<AuthHeader>) -> Html<String> {
    Html(format!("request id: {}", auth.id))
}

#[instrument]
async fn request_with_id() {
    tokio::time::sleep(std::time::Duration::from_millis(1500)).await;

    let resp = reqwest::Client::new()
        .get("http://127.0.0.1:3000/request-id")
        .header("x-request-id", "1234")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    println!("{}", resp);

    let resp = reqwest::Client::new()
        .get("http://127.0.0.1:3000/request-id")
        .header("x-request-id", "bad")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    println!("{}", resp);
}

#[derive(Debug, Clone)]
struct AuthHeader {
    id: String,
}

async fn auth(
    headers: HeaderMap,
    mut req: Request,
    next: Next,
) -> Result<impl IntoResponse, (reqwest::StatusCode, String)> {
    if let Some(h) = headers.get("x-request-id") {
        let header_data = h.to_str().unwrap();
        if header_data == "1234" {
            req.extensions_mut().insert(AuthHeader {
                id: header_data.to_string(),
            });
            return Ok(next.run(req).await);
        }
    }
    Err((StatusCode::UNAUTHORIZED, "invalid header".to_string()))
}

async fn compressed_file() -> impl IntoResponse {
    const WAR_AND_PEACE: &str = include_str!("../../../../warandpeace.txt");
    Html(WAR_AND_PEACE)
}

#[instrument(level = "info")]
async fn tracing_check() -> Html<&'static str> {
    tracing::info!("serving trace");
    Html("<h1>TRACING!</h1>")
}

#[derive(Serialize, ToSchema)]
struct HelloWorld {
    message: String,
}

#[utoipa::path(
    get,
    path = "/",
    responses(
        (status = 200, description = "Say Hello to the World", body = [HelloWorld])
    )
)]
async fn swagg_handler() -> Json<HelloWorld> {
    Json(HelloWorld {
        message: "Hello, World!".to_string(),
    })
}
