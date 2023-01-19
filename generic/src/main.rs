/// 泛型可以在结构体、枚举、方法和trait中使用。
/// 泛型代表一种泛化的思想，在其他编程语言中也有
/// 但是rust的泛型和trait配合，可以实现多态等面向对象的灵活操作
/// 
/// 通过where或者写在前面的<T:Clone>对泛型进行约束，很像java中的<? extends x>
/// 而对于trait部分的内容在trait章节进行展开，这里主要看泛型的用法。

// 结构体中使用泛型
struct Point<T,U> {
    x: T,
    y: U,
}
// 枚举中使用泛型
enum Option<T> {
    Some(T),
    None,
}
// 普通方法中使用泛型
fn echo<T>(a: T) -> T {
  return a;
}
// 在trait中使用泛型
trait Echo<T> {
    fn echo(&self, a: T) -> T;
}


// 泛型结构体实现普通的trait
trait NomalTrait {
    fn hello();
}
impl <T, U> NomalTrait for Point<T, U> {
    fn hello() {
        println!("hello");
    }
}

// 泛型结构体实现泛型trait，其中Echo的T就是Point中的T类型
//  这里使用了where进行泛型的trait约束，表示T必须是实现了Clone的类型
//  这样可以在后面调用clone方法
//  如果Point中的第一个元素没有实现这两个trait，那么Point也就没有实现Echo
impl <T,U> Echo<T> for Point<T, U> 
    where T: Clone
{
    fn echo(&self, a: T) -> T {
        self.x.clone()
    }
}


// Point实现了Echo，还可以指定他的引用也实现这个trait，并不矛盾
impl <T, X, Y> Echo<T> for &Point<X, Y> 
{
    fn echo(&self, a: T) -> T {
        a
    }
}


fn main() {
    println!("Hello, world!");
}
