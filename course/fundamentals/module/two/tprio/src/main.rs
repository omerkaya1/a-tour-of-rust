use std::{sync::atomic::AtomicI32, time::Duration};

use thread_priority::*;

static HIGH_COUNT: AtomicI32 = AtomicI32::new(0);
static LOW_COUNT: AtomicI32 = AtomicI32::new(0);
static MEDIUM_COUNT: AtomicI32 = AtomicI32::new(0);

fn low() {
    set_current_thread_priority(ThreadPriority::Min).unwrap();
    loop {
        LOW_COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        std::thread::yield_now();
    }
}

fn medium() {
	loop {
		MEDIUM_COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
		std::thread::yield_now();
	}
}

fn high() {
	set_current_thread_priority(ThreadPriority::Max).unwrap();
	loop {
		HIGH_COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
		std::thread::yield_now();
	}
}

fn main() {
	std::thread::spawn(low);
	std::thread::spawn(medium);
	std::thread::spawn(high);

	std::thread::sleep(Duration::from_millis(150));
	println!("Low {}", LOW_COUNT.load(std::sync::atomic::Ordering::Relaxed));
	println!("Medium {}", MEDIUM_COUNT.load(std::sync::atomic::Ordering::Relaxed));
	println!("High {}", HIGH_COUNT.load(std::sync::atomic::Ordering::Relaxed));
}
