// 函数基本结构
fn sum(a: i32, b: i32) -> i32 {
    return a + b;
}

fn main() {
    // 变量声明
    let a = 1;
    let b: f32 = 20.0;
    let c = "Hello World";
    let al = [1, 2, 3];
    println!("a: {}, b: {}, c: {}, al: {:?}", a, b, c, al);

    // 可变变量
    let mut d = 1;
    println!("d: {}", d);
    d = 2;
    println!("d: {}", d);

    // 重影
    let e = 1;
    println!("e: {}", e);
    let e = e + 1;
    println!("e: {}", e);

    // 函数调用
    println!("1 + 2 = {}", sum(1, 2));
}
