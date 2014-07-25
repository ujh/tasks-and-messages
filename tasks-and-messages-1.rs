use std::rand::random;

fn montecarlopi(n: uint) -> f32 {
    let mut m = 0u;
    for _ in range(0u, n) {
        let x = random::<f32>();
        let y = random::<f32>();
        if (x*x + y*y) < 1.0 {
            m = m + 1;
        }
    }
    4.0 * m.to_f32().unwrap()/n.to_f32().unwrap()
}

fn main() {
    println!("For       1000 random drawings pi = {}", montecarlopi(1000));
    println!("For      10000 random drawings pi = {}", montecarlopi(10000));
    println!("For     100000 random drawings pi = {}", montecarlopi(100000));
    println!("For    1000000 random drawings pi = {}", montecarlopi(1000000));
    println!("For   10000000 random drawings pi = {}", montecarlopi(10_000_000));
}
