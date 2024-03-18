use rand::Rng;
fn main() {
    // with_back();
    // with_back_2();
    random_hex();
}
fn _with_back() -> () {
    let mut x = 1;
    println!("{}", x);
    x = 2;
    println!("{}", x)
}
fn _with_back_2() {
    let x = 1;
    let y = 2;
    let _c = 3;
    println!("x = {:?}, y = {:?}", x, y)
}
fn random_hex() -> () {
    //random hex
    let mut rng = rand::thread_rng(); // 提供了一个生成器实例
    let ru8: u8 = rng.gen(); // 根据上下文生成 对应 的 随机数
    let ru32: u32 = rng.gen();
    let rbool: bool = rng.gen();
    let rf64: f64 = rng.gen();
    println!("Random u8-> {}", ru8);
    println!("Random u32-> {}", ru32);
    println!("Random rbool-> {}", rbool);
    println!("Random rf64-> {}", rf64);

}
