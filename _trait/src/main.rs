use std::fmt::Debug;

/// trait 的声明与基本使用，一定记住的是特征只是一种对泛型的约束，重心还在泛型
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

/// 特征约束，特征与接口很像，但是也有重要的不同，就是trait不是一种类型，不能被函数作为参数类型的，
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

/// 重点来了，f1-f4 trait作为入参，其实本质可以传任何实现了该trait的类型，即这个函数底层会编译成很多份。
/// 但是作为返回值则不可以，原因是返回值如果是动态的，例如A和B结构体都实现了trait，那么if一个条件返回A，否则返回B
/// 那么返回的类型就是不确定的，调用方的栈上就不知道申请多大的空间了，因而不能在返回值类型使用泛型+trait
// fn f5<T>() -> T 
// where T: Say + Debug {
//     User {id: 1, name: "foo".to_string()}
// }

/// 没有trait约束的泛型则可以作为返回值，例如f5中返回的类型是固定的T，其占用的内存与传入的t一致。
fn f5<T>(t: T) -> T {
    t
}

/// 针对trait作为返回值，就只能在堆上申请空间了，解决方法叫做特征对象。特征对象有两种写法Box<dyn trait>和&dyn trait
/// Box<dyn trait>实际上内部有两个指针：指向真正的结构体user的指针，和指向vTable虚函数列表的指针，这个函数列表只包含trait中定义的函数再user内的实现，无其他。
/// dyn是一种特殊的类型，只能配合Box和&使用，且dyn只能用一个trait修饰，不能A+B，如果想要A+B可以写个C trait继承AB

fn f6() -> Box<dyn Say> {
    Box::new(User{id:1, name: "foo".to_string()})
}

/// 引用作为返回值的时候，有一些生命周期的限制，不过作为入参就可以了.
///     特征对象作为入参意义其实不大，因为普通的泛型+trait能实现该功能
fn f7(sayer: &dyn Say) {
    sayer.say_hello();
}

fn f8(sayer: &impl Say) -> &dyn Say {
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


fn main() {
    let user = User {id: 1, name: "foo".to_string()};
    f4(user);

    let user = User {id: 1, name: "foo".to_string()};
    let res = f8(&user);
    res.say_hello();
}

