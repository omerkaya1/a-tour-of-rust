use shared_data::{
	CollectorCommandV1,
	DATA_COLLECTION_ADDRESS,
};
use std::{
	sync::mpsc::Sender, time::Duration, time::Instant};

pub fn collect_data(tx: Sender<CollectorCommandV1>) {
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
		let cpu_usage_total = sys.cpus().iter().map(|c| {
			c.cpu_usage()
		}).sum::<f32>();
		let cpu_usage_avg = cpu_usage_total / num_cpu as f32;

		// data submission
		let send_result = tx.send(CollectorCommandV1::SubmitData { 
			collector_id: 0, 
			total_memory: mem_total, 
			used_memory: mem_used_total, 
			average_cpu_usage: cpu_usage_avg, 
		});
		if let Err(e) = send_result {
			// pooe man's error logging
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
