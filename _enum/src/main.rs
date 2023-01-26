/// 1 与java类似可以区分多种子类型，通过A::T1来使用它
enum A {
    T1,
    T2,
    T3,
}
/// 2 每一种类型可以是一个结构体，此时是一个动态结构体类型。枚举变量的内存会以最大的变体为准
///     java中枚举是静态的，rust可以是动态的结构体
#[derive(Debug)]
enum B {
    T1(u8, i32), // 元组结构体
    T2{ id: i32, name: String}, // 普通结构体
    T3,          // 没有结构体，只是一种变体
}



fn main() {
    // 3 match来判断结构体的具体变体，match操作所有权变量会剥夺所有权
    //  在match分支中剥离结构体内部成员也会取得所有权
    let b = B::T1(1, 2);

    // match的分支中，结构体内字段类型不要显式声明，让编译器自动判断
    match b {
        B::T1(x, y) => println!("T1 {} {}", x, y),
        B::T2 {id, name } => println!("T2 id={} name={}", id, name),
        B::T3 => println!("T3"),
    };

    // println!("{:#?}", b); // match将b moved，因而无法再使用b了
    let b = B::T1(1, 2);

    // 4 如果match是对引用进行匹配判断，那么就不会获取所有权，并且对于结构体内的成员也是引用类型
    // 例如下面x是&u8类型，id是&i32类型，name是&string类型
    match &b {
        B::T1(x, y) => println!("T1 {} {}", x, y),
        B::T2 {id, name } => println!("T2 id={} name={}", id, name),
        B::T3 => println!("T3"),
    };

    println!("{:#?}", b);  // match &b就没有moved，所以可以继续使用b，match对引用类型做判断是建议的写法。

    // 5 如果变体太多，可以用_代替其他，类似java中的default
    match &b {
        B::T1(x, y) => println!("T1 {} {}", x, y),
        _ => println!("Not T1"),
    };

    // 6 如果只想对一个分支进行处理，其他分支跳过，则可以用if let
    if let B::T1(x, y) = b {
        println!("T1 {} {}", x, y);
    }


}
