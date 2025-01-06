fn main() {
    let core_ids = core_affinity::get_core_ids().unwrap();

    let handles = core_ids.into_iter().map(|id| {
        std::thread::spawn(move || {
            let success = core_affinity::set_for_current(id);

            if success {
                println!("thread on core {id:?}");
                return;
            }
            println!("unable to set affinity {id:?}")
        })
    }).collect::<Vec<_>>();    

    for h in handles {
        h.join().unwrap();
    }
}
