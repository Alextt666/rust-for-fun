// fn main() {
//     let t = (String::from("hello"), String::from("world"));

//     let _s = t.0;

//     // 仅修改下面这行代码，且不要使用 `_s`
//     println!("{:?}", t.1);
//  }

// fn main() {
//     let t = (String::from("hello"), String::from("world"));

//     // 填空，不要修改其它代码
//     let (s1, s2) = t.clone();

//     println!("{:?}, {:?}, {:?}", s1, s2, t); // -> "hello", "world", ("hello", "world")
// }

// fn main() {
//     let x = 5;
//     // 填写空白处
//     let p = &x;

//     println!("x 的内存地址是 {:p}", p); // output: 0x16fa3ac84
//  }

// fn main() {
//     let x = 5;
//     let y = &x;

//     // 只能修改以下行
//     assert_eq!(5, *y);
// }

// 修复错误
// fn main() {
//     let mut s = String::from("hello, ");

//     borrow_object(&mut s)
// }

// fn borrow_object(s: &mut String) {
//     println!("{}",s)
// }

// // 修复错误
// fn main() {
//     let mut s = String::from("hello, ");

//     push_str(&mut s)
// }

// fn push_str(s: &mut String) {
//     s.push_str("world")
// }

// fn main() {
//     let mut s = String::from("hello, ");

//     // 填写空白处，让代码工作
//     let p = &mut s;

//     p.push_str("world");
// }

// fn main() {
//     let c = '中';

//     let r1 = &c;
//     // 填写空白处，但是不要修改其它行的代码
//     let ref r2 = c;

//     assert_eq!(*r1, *r2);

//     // 判断两个内存地址的字符串是否相等
//     assert_eq!(get_addr(r1),get_addr(r2));
// }

// // 获取传入引用的内存地址的字符串形式
// fn get_addr(r: &char) -> String {
//     format!("{:p}", r)
// }

// closure
fn main() {
    fn f1() {
        let s1 = String::from("hello");
        println!("{}", &s1);
        let closure = || println!("{}", &s1);
        closure();
    }
    f1()
}
