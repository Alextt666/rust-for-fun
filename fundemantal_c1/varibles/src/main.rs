/*
    Is this varible mutable
    变量手动选择是否可变 (添加mut)
    报错  cannot assign twice to immutable variable `x`
    ```
    let x = 5;
    println!("The value of x is: {}",x);
    x = 6;
    println!("The value of x is: {}",x);
    ```
    正确版本
    ```
    let mut x = 5;
    println!("The value of x is: {}",x);
    x = 6;
    println!("The value of x is: {}",x);
    ```
*/
fn main() {
    // let x = 5;
    // println!("The value of x is: {}", x);
    // x = 6;
    // println!("The value of x is: {}", x);

    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}
