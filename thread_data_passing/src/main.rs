pub mod mine;

use std::{sync::mpsc::channel, thread, time::Duration};

use mine::Detail;
use rand::Rng;

fn main() {
    let (tx, rx) = channel();

    // let mut _join_handles: Vec<JoinHandle<_>> = Vec::new();

    let d = mine::Details {
        name: "Raj".to_string(),
        age: 16,
    };

    d.get_details();

    let mut rng_m = rand::thread_rng();

    for _ in 1..5 {
        let delay = rng_m.gen_range(1000..2000);
        let tx_clone = tx.clone();
        let _ = thread::spawn(move || {
            for j in 1..5 {
                thread::sleep(Duration::from_millis(delay));
                tx_clone
                    .send(format!("I did my job no: {:} with delay of {:}", j, delay))
                    .unwrap();
            }
        });
        // _join_handles.push(handle)
    }

    for received in rx {
        println!("{:}", received.to_string());
    }

    // for handle in join_handles {
    //     handle.join().unwrap();
    // }
}
