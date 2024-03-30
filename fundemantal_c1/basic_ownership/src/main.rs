/*
    ownership 建立在 内存 堆栈管理的基础上:
    Rust 中每一个值都被一个变量所拥有，该变量被称为值的所有者
    一个值同时只能被一个变量所拥有，或者说一个值只能拥有一个所有者
    当所有者(变量)离开作用域范围时，这个值将被丢弃(drop)
    可以称 堆内存数据的所有权转移为Move
    基本数据类型 是copy  因为 发生在栈内存之上 所以 直接copy 速度非常快
*/
// 不要修改 main 中的代码
fn main() {
    let s1 = String::from("hello, world");
    let s2 = take_ownership(s1);

    println!("{}", s2);
}

// 只能修改下面的代码!
fn take_ownership(s: String)->String {
    println!("{}", s);
    s
}

/*
    give_ownership - s1
    创建一个字符串 将所有权给出
    s2 - 直接创建 新的字符串
    takes_and_gives_back - s3
    获取字符串所有权 再给出
    ```
        let s1 = give_ownership(); // 函数将所有权返回给s1
        let s2 = String::from("s2-text"); // s2 拥有"s2-text"的所以权
        println!("{}",s2); // 可以打印 s2
        let s3 = takes_and_gives_back(s2); // s2 所有权 先移交给 函数 后返回 给s3 -> s2 丧失所有权
        println!("{},{}",s1,s3); // 可以打印s1-s3 但 无法打印s2 因为s2的所有权丧失
    ```
*/

fn excute(){
    let s1 = give_ownership();
    let s2 = String::from("s2-text");
    println!("{}",s2);
    let s3 = takes_and_gives_back(s2);
    println!("{},{}",s1,s3);
}

fn give_ownership() -> String {
    let some_string = String::from("alex_something");
    some_string
}
fn takes_and_gives_back(a_string: String)-> String{
    a_string
}