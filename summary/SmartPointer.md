# 智能指针
智能指针就是实现了`Deref`和`Drop`的结构体，实现了这两个trait，就可以像指针一样使用该结构体了。`Deref`使得结构体能使用`*`这个解引用操作符，来下钻指针指向的内存数据，而`Drop`则可以在指针变量离开作用域的时候，重写drop方法，来删除自己管理的地址的数据。

rust中`&`取地址符的作用是获取变量的引用，虽然引用本质也是地址，但是在rust中引用有特殊的含义，没有数据的所有权，要符合借用的条约。

常见智能指针：
- String是str的智能指针，Vec<T>是[T]的智能指针，这俩都是变长类型，本质都是堆上的数组，用指针存储堆地址和相关的元数据信息。
- Box<T>在堆上创建数据，Box存储指向数据的地址，类似`malloc`的作用。Box还用于特征对象等常见场景。
- Rc<T>也是堆上创建，是Box的增强，clone方法使得数据可以被多个变量持有所有权，打破单一owner规则，当引用数归零时删除T。
- Ref<T>与RefMut<T>是RefCell调用borrow/borrow_mut的返回值，RefCell提供了内部可变性，其自身不是智能指针，这俩才是智能指针。与之相似的Cell也不是智能指针，也只是通过Ref、RefMut修改数据的。
- MutexGuard<T>是Mutex<T>互斥锁加锁后得到的智能指针，具有可变性。
- Cow<T>写时克隆，适用于作为这样的函数返回类型，处理入参且有可能不需要处理直接返回入参的场景，做到惰性clone下面介绍。

# Box
Box是在堆上申请空间，Box结构体内存储地址，因而Box是`Sized`。
```rs
// 将1创建到堆上，b是指向1的指针。
let mut b = Box::new(1);

// 解引用，因为i32是Copy的，所以可以直接拿出来。如果是非Copy的，则不能直接解引用成T类型使用，因为不能直接获取所有权
let c: i32 = *b;

// 解引用另一个重要作用是用在等号左侧对内部数据的赋值，以下方式效果相同
*b = 11;
*b.as_mut() = 12;

// 智能指针可以直接调用&T的方法，这是自动解引用，本质是函数入参的自动解引用。Box能自动解为&T类型
let s = Box::new("1".to_string());
let l = s.len();
```
因为Box的大小是固定的指针大小，所以常用来作为动态结构的包装。
```rs
// 1 结构体的嵌套，如果是结构体类型是无法估计嵌套的最终大小的，Box包裹后只有指针特定大小就解决了无限大结构体问题
struct Person {
    id: i32,
    // Box<Option>和Option<Box>都可以工作，但是前者更好，因为Box是智能指针可以直接当Option来使用其内部函数
    parent: Box<Option<Person>>
}

// 2 作为函数返回值的特征对象，函数返回值如果特征类型，是不能直接用泛型约束的，原因在trait章节有解释，需要改为用特征对象
fn f() -> Box<dyn Debug> {...}
```
关于智能指针的函数入参如何选型，上面说了自动解引用功能，这样我们写函数的时候，入参类型最好就写智能指针内包裹的类型`&T`。
```rs
// String是str的智能指针，写&str也可以接收&String类型，具有更好的灵活度。
fn f(s: &str) {}

// 同理Box也不要出现在函数的入参类型中
fn f(s: Box<i32>){} // 这样写不好，应改为下面
fn f(s: &i32){}

// 特征对象一般作为返回值，在入参中虽然也能工作但是不太好，因为dyn是动态分发性能比impl要低
fn f(s: Box<dyn Debug>) {} // 这样写不好
fn f(s: &dyn Debug) {} // 这样写不好，改为下面
fn f(s: &impl Debug) {}

// 作为返回值，是不得不用dyn所以才这么写的
fn f() -> Box<dyn Debug>
```
# Rc / Arc
引用计数，可以让多个地方都持有变量所有权，但是Rc/Arc有一条重要原则：不能对T进行写操作。
```rs
let a = Rc::new(1);
// u1 u2 同时持有a。
let u1 = User { id: a.clone()};// 最好用Rc::clone(&a)，防止调错了clone方法。
let u2 = User { id: a.clone()};// 最好用Rc::clone(&a)，防止调错了clone方法。
```
`AsRef`与`AsMut`，一般来说智能指针会实现这俩trait来获取内部T的引用，像Rc这种不支持写操作的就只实现`AsRef`
```rs
let a = Rc::new(1);
let b: &i32 = a.as_ref();

let a = Box::new(1);
let b: &i32 = a.as_ref();
let c: &mut i32 = a.as_mut();
```
`Arc`是`Rc`的多线程版本，在线程传递时需要用`Arc`
# RefCell
`Rc<RefCell>`是常见的搭配，既有多主又有内部可变。`RefCell`底层记录了对T的引用和可变引用数量，在运行时保证，同时只能1个`RefMut`或者多个`Ref`的准则，编译时不再检查。
```rs
let a = Rc::new(RefCell::new(1));
let rf: RefMut<i32> = a.borrow_mut();
*rf = 2;
```
# Mutex
`Arc<Mutex>`是`Rc<RefCell>`的多线程版本，因为`RefCell`很有可能在多线程场景下出现多个`RefMut`，所以需要有锁的保护，于是就有了`Mutex`，智能指针是`Mutex.lock.unwrap`出的`MutexGuard`
```rs
let a = Arc::new(Mutex::new(1));

let l = a.clone();
thread::spawn(move || {
    {
        // 互斥的部分是 m 所在的作用域
        let mut m: MutexGuard<i32> = l.lock().unwrap();
        *m = *m + 1;
    }
});
```
# Cow
Clone On Write写的时候clone，换句话说就是读场景就不需要任何操作。`Cow`一般配合切片类型来使用，尤其是`String`和`str`。其本身是个枚举，对于`B`和`B::Owned`类型的变体，通俗举个例子就是里面可以放`&str`也可以放`String`。`Cow`有一定的作用，但是不是必须的，节省一定的开销。
```rs
pub enum Cow<'a, B>
where
    B: 'a + ToOwned + ?Sized,
 {
    Borrowed(&'a B),
    Owned(<B as ToOwned>::Owned),
}
```
下面例子中将返回值从`String`改为`Cow<str>`，可以避免当不含有空格的时候，重新创建String变量。`Cow`因为是智能指针，拥有`str`的各种方法，可以直接拿来用。
```rs
// 正常的删除空格方法，调用str的replace就可以返回String
fn rm_space(s: &str) -> String {
    let res: String = s.replace(" ","");
    res
}

// 使用Cow<str>作为返回值，可以通过String.into或&str.into得到Cow
fn rm_space(s: &str) -> Cow<str> {
    if s.contains(" ") {
        let res: String = s.replace(" ","");
        return res.into();
        // 或者写成 return Cow::Owned(res)
    }
    s.into()
    // 或者写成 return Cow::Borrowed(res)
}
```