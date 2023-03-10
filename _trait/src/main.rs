use std::{fmt::{Debug, Display}, str::FromStr, error::Error, num::ParseIntError};

/// 1 trait 的声明与基本使用，一定记住的是特征只是一种对泛型的约束，重心还在泛型
trait Say {
    fn say_hello(&self);
    fn give_me_five(&self) -> i32 {
        5
    }
}

#[derive(Debug)]
struct User {
    id: i32,
    name: String,
}

impl Say for User {
    fn say_hello(&self) {
        println!("Hello! I am {}", self.name);
    }
}

/// 2 特征约束，特征与接口很像，但是也有重要的不同，就是trait不是一种类型，不能被函数作为参数类型的，
///     特征只能作为泛型的约束来修饰函数出参和入参，例如在入参添加impl trait修饰才代表是实现了Say的类型
///     impl是普通函数约束写法的语法糖
fn f1(sayer : impl Say) {
    sayer.say_hello();
}
/// f1是f2的语法糖
fn f2<T: Say>(sayer: T) {
    sayer.say_hello();
}
/// f3是f2的where写法，可以让函数签名处短一点。f1 f2 f3等价
fn f3<T>(sayer: T) 
where T: Say {
    sayer.say_hello();
}

/// 多个trait约束
fn f4<T>(sayer: T) 
where T: Say + Debug {
    sayer.say_hello();
}

/// 3 重点来了，f1-f4 trait作为入参，其实本质可以传任何实现了该trait的类型，即这个函数底层会编译成很多份。
/// 但是作为返回值则不一定可行，原因是返回值如果是动态的，例如A和B结构体都实现了trait，那么if一个条件返回A，否则返回B
/// 那么返回的类型就是不确定的，调用方的栈上就不知道申请多大的空间了，因而不能在返回值类型使用泛型+trait

// 这个写法是错误的，因为期望返回的是T类型，编译器不知道有没有在除了返回类型的其他地方使用T
// 所以直接返回User类型是不对的，即使我们的User确实也实现了Say+Debug
// fn f5<T: Say+Debug>() -> T {
//     User{id:1, name:"foo".to_string()}   
// }

// 这个写法是正确的，因为impl虽然是上面的简写，但是impl保证了T只出现在这里
fn f5() -> impl Say+Debug {
    User{id:1, name:"foo".to_string()}
}

// 这个写法是正确的，因为返回值的类型在编译器能确定就是User
fn f5_1() -> impl Debug {
    if (100>99) {
        User{id:1, name:"foo".to_string()}
    } else {
        User{id:2, name:"bar".to_string()}
    }
}

// 这个写法是错误的，因为返回值的类型
// fn f5_2() -> impl Debug {
//     if (100>99) {
//         "string".to_string()
//     } else {
//         User{id:2, name:"bar".to_string()}
//     }
// }


// 这样写是可以的，因为t一定是T类型
fn f6<T: Say+Debug>(t: T) -> T {
    t
}

/// 针对动态的trait类型作为返回值像f5_2，就只能在堆上申请空间了，解决方法叫做特征对象。特征对象有两种写法Box<dyn trait>和&dyn trait
/// Box<dyn trait>实际上内部有两个指针：指向真正的结构体user的指针，和指向vTable虚函数列表的指针，这个函数列表只包含trait中定义的函数再user内的实现，无其他。
/// dyn是一种特殊的类型，只能配合Box和&使用，且dyn只能用一个trait修饰，不能A+B，如果想要A+B可以写个C trait继承AB

fn f7() -> Box<dyn Say> {
    Box::new(User{id:1, name: "foo".to_string()})
}

/// 引用作为返回值的时候，有一些生命周期的限制，不过作为入参就可以了.
///     特征对象作为入参意义其实不大，因为普通的泛型+trait能实现该功能
fn f8(sayer: &dyn Say) {
    sayer.say_hello();
}

fn f9(sayer: &impl Say) -> &dyn Say {
    sayer
}

trait A {
    // 如果要返回一个自己这个类型
    // 可以用特征对象或者Self，如下两个，但是两者不能共存，会打破对象安全性规则
    // Self和dyn不能同时作为返回值的，Self指向真实对象，dyn化后编译器无法知晓真实的类型
    // https://doc.rust-lang.org/reference/items/traits.html#object-safety
    // fn generate1(&self) -> Box<dyn A>;

    fn generate2(&self) -> Self;
    
}

/// 4 关联类型，是指在trait中首行声明一个type xx，并且后面使用Self::xx代表这个类型
/// 例如Iterator中的Item。
/// 
/// pub trait Iterator {
/// type Item;
/// 
/// fn next(&mut self) -> Option<Self::Item>;
/// }
/// 
/// 关联类型可以直接用泛型所代替例如trait Iterator<T>{fn next(&mut self)->Option<T>}
/// 效果是一样的，但是关联类型的写法，更加简洁，比如当trait中的函数需要多次使用type，
/// 再比如当trait被拿去约束泛型的时候，这个trait里需要写<泛型>的，而关联类型就不需要
/// 


/// 5 对于同一个struct实现多个特征且具有同名方法时，不要用struct.func调用，容易不知道调用的哪个
///     而是使用Trait::func(&struct)

fn main() {
    let user = User {id: 1, name: "foo".to_string()};
    f4(user);

    let user = User {id: 1, name: "foo".to_string()};
    let res = f9(&user);
    res.say_hello();

    let a = "123";
    let x = a.to_string().parse::<i32>();
    i32::from_str("111").unwrap();
    
    a.to_string();
    let s = String::from(a);
    
    // let s: &str = s.into();
}

/// 6 trait的继承用T2:T1表示继承T1特征，如果一个结构体要实现T2，那么需要同时实现T1才行
struct T {}
trait T1 {
    fn t1_f(&self) {}
}
trait T2: T1 {
    fn t2_f(&self) {}
}
impl T1 for T {
    fn t1_f(&self) {}
}
impl T2 for T {
    fn t2_f(&self) {}
}
/// 7 在trait中无法将函数的返回值设置为impl xx，
/// 但是可以通过Self返回当前结构体类型，关联类型+triat返回其他类型
trait T3 {
    type X: Debug;
    
    fn fn_a(&self) -> Self;

    fn fn_b(&self) -> Self::X;
}