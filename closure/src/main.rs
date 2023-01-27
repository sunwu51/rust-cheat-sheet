use std::thread;

/// 闭包与函数
/// fn Fn FnMut FnOnce
/// fn是函数关键字，后面三个是针对闭包的trait
/// fn实现了Fn，Fn实现了FnMut，FnMut实现了FnOnce，如果一个函数接收参数是impl FnOnce，那么这四种都可以传入
/// 如果是impl FnMut则可以传入除FnOnce外其他三种

fn main() {
    let mut x = "123".to_string();
    // 1 f1中没有对上下文变量的捕捉，所以f1本质是fn。fn是可以Copy的所以下面多次使用f1是对f1的Copy
    let f1 = || println!("hello");
    ff1(f1);
    ff2(f1);
    ff3(f1);
    ff4(f1);

    // 2 f2引用了上下文变量x，但是只要不可变引用即可，所以是Fn。Fn也是Copy的
    let f2 = || println!("{}", x);
    ff2(f2);
    ff3(f2);
    ff4(f2);

    // 3 f3引用了上下文变量x，但是只要可变引用即可，所以是FnMut。FnMut和FnOnce就不是Copy的，原因很显然，Copy两份不就俩mut ref了或者俩owner了
    let f3 = || { x.push_str("456"); println!("{}", x)};
    ff3(f3);
    // 不能copy所以再create一个
    let f3 = || { x.push_str("456"); println!("{}", x)};
    ff4(f3);

    // 4 f4中y夺取x所有权了，所以是FnOnce
    let f4 = || {let y = x; println!("{}", y); };
    ff4(f4);

    // 5 关于move，如果遇到lifetime问题，例如上下文变量x只能活到当前函数结束
    //  但是闭包可能作为返回值(下面rf1234的例子)，或者作为回调函数时。则需要强制获取所有权，即使当前不是FnOnce
    //  f5是Fn，但加了move后不再是Copy的，并且获取了x的所有权，x不能再使用
    let x = "123".to_string();
    let f5 = move || println!("{}", x); 
    // println!("{}", x); 出错，即使f5中并不需要move x，但是move关键字仍会起作用
    ff2(f5);

    // 6 线程是典型的闭包使用的例子， spawn接收FnOnce类型，一般我们可能传入的是Fn或者FnMut的闭包，
    // 此时就会出现生命周期线程是长于ctx，所以就需要强制move，这也是最常见的move场景。此时不用move，会报错，因为x命不够长
    // 而闭包不能自己退化为FnOnce，这也是move关键字的最大作用
    let x = "123".to_string();
    thread::spawn(move || println!("{}", x));
}

// 1 fn作为参数直接写fn(入参类型)->返回类型，fn不是trait而是一种类型.->()可以不写
fn ff1(f: fn() -> ()) {f();}

// 2 Fn FnMut FnOnce是trait不是类型，需要用特征约束或者impl语法糖，或者特征对象(堆上)
//      只不过这几个trait长得不太一样别人都是Trait名，他们是Fn后面还得有(入参)，可能还有->返回类型
fn ff2(f: impl Fn()) {f();}
fn ff22<T: Fn()>(f: T) {f();}
fn ff222(f: Box<dyn Fn()>) {f();}

fn ff3(mut f: impl FnMut()) {f();}
fn ff4(f: impl FnOnce()) {f();}

// 3 作为返回值，因为特征约束不能直接用于返回类型的约束，需要用特征对象dyn（参考trait章节介绍）
fn rf1() -> fn() -> () {|| println!("hello")}
fn rf2() -> Box<dyn Fn() -> ()> {
    let x = "123".to_string();;
    Box::new(move || println!("{}", x))
}
fn rf3() -> Box<dyn FnMut() -> ()> {
    let mut x = "123".to_string();
    Box::new(move || {x.push_str("456"); println!("{}", x)})
}
fn rf4() -> Box<dyn FnOnce() -> ()> {
    let mut x = "123".to_string();
    Box::new(move || {let y = x;  println!("{}", y)})
}