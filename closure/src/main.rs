/// 闭包，闭包可以当做函数fn也可以实现FnOnce，FnMut或者Fn三种trait
/// 具体实现了哪一种需要看，捕捉的变量以何种形式能满足函数需要

fn main() {
    let x = "123".to_string();
    // 1 f1中没有对上下文变量的捕捉，所以f1本质是fn
    let f1 = |a| println!("{}", a);
    f1(&x);

    // 2 f2引用了上下文变量x，但是只要不可变引用即可，所以是Fn
    let f2 = || println!("{}", x);

    // 3 f3引用了上下文变量x，但是只要可变引用即可，所以是FnMut
    let f3 = || println!("{}", x + "456");

    
    let f4 = || return x;

}
