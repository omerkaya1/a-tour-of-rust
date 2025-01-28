fn simple_pool() {
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(4)
        .build()
        .unwrap();

    pool.spawn(|| println!("hi from witin a thread"));

    pool.scope(|scope| {
        for n in 0..20 {
            scope.spawn(move |_| {
                println!("hi from the scope {n}");
            });
        }
    });

    println!("hi from the main thread")
}

fn broadcast() {
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(4)
        .build()
        .unwrap();

    pool.scope(|scope| {
        scope.spawn_broadcast(|_scope_ctx, brodacast_ctx| {
            println!("hello from broadcast {}", brodacast_ctx.index());
        });
    });

    println!("hi from the main thread")
}

fn tester() {
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(4)
        .build()
        .unwrap();

    pool.join(test, test);
}

fn test() {
    println!("test!")
}

fn main() {
    simple_pool();

    broadcast();

    tester();
}
