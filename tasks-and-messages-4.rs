use std::io::Timer;
use std::rand::random;

fn montecarlopi(n: uint, sender: Sender<uint>) {
    println!("montecarlopi(): Starting calculation");
    let mut m = 0u;
    for _ in range(0u, n) {
        let x = random::<f32>();
        let y = random::<f32>();
        if (x*x + y*y) < 1.0 {
            m = m + 1;
        }
    }
    println!("montecarlopi(): Calculation done");
    sender.send_opt(m);
}

fn worker(receive_from_main: Receiver<uint>, send_to_main: Sender<f32>) {
    let mut m = 0u;
    let n = 10_000_000;
    let mut i = 0;
    let (sender, receive_from_montecarlo) = channel();
    let initial_sender = sender.clone();
    spawn(proc() {
        montecarlopi(n, initial_sender);
    });
    loop {
        select! {
            _ = receive_from_main.recv() => {
                println!("worker(): Aborting calculation due to signal from main");
                break;
            },
            montecarlopi_result = receive_from_montecarlo.recv() => {
                m = m + montecarlopi_result;
                i = i + 1;
                let sender_clone = sender.clone();
                spawn(proc() {
                    montecarlopi(n, sender_clone);
                });
            }
        }
    }
    let val = 4.0 * m.to_f32().unwrap()/(n*i).to_f32().unwrap();
    send_to_main.send(val);
}

fn main() {
    let mut timer = Timer::new().unwrap();
    let (send_from_worker_to_main, receive_from_worker) = channel();
    let (send_from_main_to_worker, receive_from_main)   = channel();
    println!("main(): start calculation and wait 10s");
    spawn(proc() {
        worker(receive_from_main, send_from_worker_to_main);
    });
    timer.sleep(10_000);
    println!("main(): Sending abort to worker");
    send_from_main_to_worker.send(0);
    println!("main(): pi = {}", receive_from_worker.recv());
}
