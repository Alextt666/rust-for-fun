fn main() {
    mutable_borrowing();
}

/*
    不可变引用
    使用一个值的引用 但不获取它的所有权 （&符号）
    但不可以通过引用对这个值进行修改
*/
#[allow(dead_code)]
fn immutable_borrowing() {
    let s1 = String::from("alex_text");

    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
}
#[allow(dead_code)]
fn calculate_length(str: &String) -> usize {
    return str.len();
}

/*
    可变引用
    使用一个值的引用 但不获取它的所有权 （&mut )
    可以通过引用对这个值进行修改 (值本身为 mut)
*/
fn mutable_borrowing() {
    let mut s1 = String::from("alex ");
    let len = cal_for_mut(&mut s1);
    println!("The length of '{}' is {}.", s1, len);
}
fn cal_for_mut(str: &mut String) -> usize {
    str.push_str("has to much sexy wives.");
    return str.len();
}
