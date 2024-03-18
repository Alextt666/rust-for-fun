/**
 * 主函数-介绍几种基本类型不同的声明方式
1. 靠程序推断
2. 手动声明 let a:i32 = 10;
3. 值后声明(可变 就添加 mut) let mut a = 10i32;
4. 值后声明 & _ let a = 10_i32;
函数声明 fn
高阶函数 接收函数作为返回值 & 参数
```
    let a = 10;
    let b: i32 = 10;
    let mut c = 10i32;
    let d = 10_i32;
```
 */

fn main() {
    // 声明 a,b,c,d 并调用 add 计算（a+b）+(c+d)

    let a = 10;
    let b: i32 = 10;
    let mut c = 10i32;
    let d = 10_i32;

    let res = add(add(a, b), add(c, d));
    println!("(a+b) + (c+d) = {}", res);
}
/**
 add-函数 计算两参数之和
 */
fn add(i: i32, j: i32) -> i32 {
    return i + j; // 不加分号 否则返回 为 ()  而非 i32;
}
