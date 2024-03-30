use std::thread;
use std::time::Duration;
fn main() {
    // 创建一个线程 
    thread::spawn(||{
        for i in 1..10 {
            println!("Thread output {}",i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5{
        println!("Main output {}",i);
        thread::sleep(Duration::from_millis(1));
    }
}
