// fn main() {
//     let t = (String::from("hello"), String::from("world"));

//     let _s = t.0;

//     // 仅修改下面这行代码，且不要使用 `_s`
//     println!("{:?}", t.1);
//  }

fn main() {
    let t = (String::from("hello"), String::from("world"));

    // 填空，不要修改其它代码
    let (s1, s2) = t.clone();

    println!("{:?}, {:?}, {:?}", s1, s2, t); // -> "hello", "world", ("hello", "world")
}
