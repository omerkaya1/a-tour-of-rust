use shared_data::CollectorCommandV1;
use std::{sync::mpsc::Sender, time::{Duration, Instant}};

pub fn get_uuid() -> u128  {
	let path = std::path::Path::new("uuid");
	if path.exists() {
		let data = std::fs::read_to_string(path).unwrap();
		return data.parse::<u128>().unwrap();
	}
	let uuid = uuid::Uuid::new_v4().as_u128();
	std::fs::write(path, uuid.to_string()).unwrap();
	uuid
}

pub fn collect_data(uid: u128, tx: Sender<CollectorCommandV1>) {
    let mut sys = sysinfo::System::new_all();

    sys.refresh_memory();
    sys.refresh_cpu_all();

    std::thread::sleep(Duration::from_secs_f32(0.5));

    loop {
        let now = Instant::now();

        sys.refresh_memory();
        sys.refresh_cpu_all();

        // obtain refreshed data
        let mem_total = sys.total_memory();
        let mem_used_total = sys.used_memory();
        let num_cpu = sys.cpus().len();
        let cpu_usage_total = sys.cpus().iter().map(|c| c.cpu_usage()).sum::<f32>();
        let cpu_usage_avg = cpu_usage_total / num_cpu as f32;

        // data submission
        let send_result = tx.send(CollectorCommandV1::SubmitData {
            collector_id: uid,
            total_memory: mem_total,
            used_memory: mem_used_total,
            average_cpu_usage: cpu_usage_avg,
        });
        if let Err(e) = send_result {
            // poor man's error logging
            println!("failure to send data to channel: {e:?}");
        }

        let mut sleep_dur = 1.0;
        let since = now.elapsed().as_secs_f32();
        if since < 1.0 {
            sleep_dur -= since;
        }
        std::thread::sleep(Duration::from_secs_f32(sleep_dur));
    }
}
