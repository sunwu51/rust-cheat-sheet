# Rust如何组织代码

# 1 Rust中的变量类型
Rust与其他语言一样又内置的基础类型，例如`bool`，`i32`，`f64`等。
```rs
let a = 1;
let b = false;
```
也有复杂的类型例如`元组`，`结构体`，`枚举`等。
```rs
let a = (1, "1");
let b = User {id: 1, name: "gg".to_string()};
```
不管是基础还是复合类型，默认都是在栈上创建，对于基础类型都实现了`Copy`，因而没有所有权的move，都是在内存上拷贝了一份新的。而对于复合类型默认都没有实现`Copy`接口，所以当传入其他作用域的时候就会发生所有权的转移。
```rs
let a = 1;
let b = a; // Copy not moved
println!("{}", a);
```
数组和切片也是常见的类型，尤其是切片，他与字符串息息相关。
```rs
// 切片是一个view，无法扩缩容，但是可以改某一下标内的值
let sli1: &mut [i32] = arr1.as_mut_slice();
```
函数也是类型的第一公民，`fn(T)->U`是常见的类型签名，可以作为其他函数的入参和返回参数类型。
```rs
// fn作为fn的入参和返回值
fn f(f: fn(i32)->String) -> fn(String)->i32 {...}
```
闭包也是类型第一公民，自动类型推断显示是`||->()`，但是我们不能显示的写明类型，也就是说我们无法表示一个闭包的类型签名。当作为函数出入参时，只能通过trait来约束闭包。

而至于为什么不能给一个闭包显式的类型签名，是因为闭包所占的空间，与函数代码长度，上下文变量等都有关系，这就使得即使出入参类型相同，两个函数所占的内存仍旧不同，一个类型如果没有固定的内存大小，那就没办法给他一个类型签名，除非扔堆上，但闭包本质是结构体显然是栈上申请的。

# 2 Rust中的类型转换
数字类型可以通过`as`做转换，避免有损的转换。
```rs
// 整数互转会截断，如下
let x  = 260;
let y = x as u8;
println!("{}", y); // 4
// 但是f64转u8如果超过255直接转为255
let x  = 260。0;
let y = x as u8;
println!("{}", y); // 255
```
对于结构体之间的互转，rust提供了`Into`与`From`特征，一般来说一个结构体Type1会主动实现`From<Type2>`特征.例如`String`中就实现了`From<&str>`这个trait。
```rs
impl From<&str> for String {
    fn from(s: &str) -> String {
        s.to_owned()
    }
}
```
有了上述`From<&str>`，就可以通过静态方法`from`转换`&str`为`String`，或者通过绑定到结构体上的`into`方法实现`&str`到`String`转换，注意转换方向不变，`from/into`只是两种写法
```rs
// from写法
let s: String = String::from("123");

// into写法，必须显式声明类型，因为可能有很多类型都实现了`From<&str>`这样就无法自动推断
let s: String = "123".into();
```
`ToOwned`特征，对一个引用实现`to_owned`方法获得一模一样的数据，并持有所有权，而非是引用，该方法的实现一般依赖`Clone`，`Clone`能从`&T`->`T`，`ToOwned`可以跨类型`&A`->`B`
```rs
let s: String = "123".to_owned();
```
对于`&str`->`String`的几种方式的概述
```
&str.to_string():   调用了String::from(&str)
String::from(&str): 调用的是str.to_owned()，所以本质都是靠to_owned
```

基础类型和`str`的转换一般基础类型都实现了`FromStr`，与`From`类似也是类型上的静态方法，与`FromStr`对称的方法是String结构体上的`fn parse<F: FromStr>(&self) -> Result<F, F::Err>`
```rs
// 字符转数字
let i:i32 = i32::from_str("1").unwrap();
let i: Result<i32, _> = "1".to_string().parse::<i32>();


// 数字转字符
// 也可以用format!
let s:String = format!("{}", 1);
```

对于`From/Into`和`FromStr/parse`小结，他们都是对称出现的type1能通过mod的静态方法`from(变量)`从一个类型转为当前类型，也可以写成`变量.into()`，转换方向一致，效果一致，不同写法而已。同理对于type1实现了`FromStr`那么就可以调用mode静态方法`from_str(&str)`来转成当前类型，也可以写成`string.parse::<type1>()`.

插一句`turboFish`语法，也就是上面parse后的`::<i32>`，是为了确定泛型内的类型用的，如果不适用涡轮鱼也可以显式指定类型，像前面的`into`。但是反之`into`那里不能用turboFish，因为不是泛型。
# 3 Rust中的面向对象
一般对于结构体实现的方法，如果不是必要情况，就用`&self`避免moved，使得原始变量不可用。
对于结构体(或枚举)可以追加方法，方法的参数定义为`self`或者`&self`，就可以通过`.`运算符直接调用。
```rs
impl User {
    fn get_name(&self) -> &str { &self.name } 
}
```
结构体没有继承，如果想要实现继承，需要通过组合。

Rust中也没有接口，只有trait，trait只能作为泛型的约束条件，来限制函数的入参，以此来达到多态的效果。

但是trait只能对入参进行约束，如果是返回类型，trait约束，会导致不同运算分支下会返回不同的实际类型，此时就需要用分配到堆上的特征对象了。

# 4 Rust中的Error处理
`panic!`不可调和的错误，直接崩溃当前线程/协程，不能被上级`try`，rust没有try机制。类似java中直接throw Exception，并且不进行try操作。

`Result<T, E>`使用更多，将处理正确的结果放到T，如果处理出现错误，则将错误放到E。一种非常通用的类型是`Result<T, Box<dyn std::error::Error>>`，使用特征对象代表所有的错误类型都可以接受。理论上E可以不是一种Error类型甚至可以是i32这种基础类型，但是为了整个代码可读性和错误生态的一致性，需要我们使用已有的实现了`std::error::Error`的结构体，或者自定义结构体。

例如我们以上面`FromStr` trait为例，写一个结构体`Point`，让他实现`FromStr`，并在实现过程中捕捉错误。
```rs
#[derive(Debug)]
struct Point{
    x:i32, y:i32
}

/// 自定义错误类型，需要实现Display和Error，Error部分我们可以用默认的实现，不做方法重写，但是Display要写。
#[derive(Debug)]
struct ParsePointError {
    msg: String
}
impl Display for ParsePointError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.msg)
    }
}
impl std::error::Error for ParsePointError {
}

/// FromStr需要指定Err类型，这里指定了万能的错误类型，因为实际代码执行的时候。`?`部分的unwrap操作可能有错误，含有逗号数量不对则会是自定义错误。
impl FromStr for Point {
    
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Point, Self::Err> {
        let c = s.split(",").collect::<Vec<&str>>();
        if c.len() == 2 {
            let x = c[0].to_owned().parse()?;
            let y = c[1].to_owned().parse()?;
            return Ok(Point{x, y});
        }
        Err(Box::new(ParsePointError{msg: "字符串格式有问题".to_owned()}))
    }
}
```
当然上面的万能错误类型其实对使用者并不友好，向上反映错误的时候，会比较难定位。可以用`derive_error`或者`thiserror`这个第三方派生宏，将自定义Error定义为enum.

下面是`derive_error`包的介绍，`thiserror`的请自行查看后续thiserror章节。枚举的每个成员必须是空或者Error，如下，可以把Other改为多个更具体的错误信息。
```rs
#[derive(Debug, Error)]
enum ParsePointError {
    ParseIntError(ParseIntError),
    Other,
}

impl FromStr for Point {
    type Err = ParsePointError;

    fn from_str(s: &str) -> Result<Point, Self::Err> {
        let c = s.split(",").collect::<Vec<&str>>();
        if c.len() == 2 {
            let x = c[0].to_owned().parse()?;
            let y = c[1].to_owned().parse()?;
            return Ok(Point{x, y});
        }
        Err(ParsePointError::Other)
    }
}
```
