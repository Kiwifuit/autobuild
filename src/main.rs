use std::thread::sleep;
use std::time::Duration;

fn main() {
    println!("Hello, world!");

    let mut counter: u32 = 0;
    loop {
        counter += 1;
        println!("I have been alive for {}s", counter);
        sleep(Duration::from_secs(1));
    }
}
