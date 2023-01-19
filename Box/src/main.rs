/// 写在前面：
/// rust的基础类型和struct都是在栈上，只有智能指针才能在堆上创建数据
/// 智能指针也是结构体，但是他的内部含有一个指向堆上数据的指针地址，可能还有别的属性
/// 最常见的智能指针就是String、Vec和Box，String Vec是常见的变长类型，他们无法在栈上创建
/// String指向堆上的Vec<u8>，Box也是指向内部的泛型格式的数据
/// ====================================================================================
/// 基本的函数返回值，基础类型、struct或智能指针，底层逻辑详细分析
///     基础类型就是在调用方的栈上copy了一份数据
/// fn f1() -> i32 {1}
/// 
///     结构体也是copy一份，如果是智能指针结构体则copy的是指向堆的地址
/// fn f2() -> String {"1".to_string()}
/// ====================================================================================
/// 堆用于数据逃逸
/// ----------------------------------------------------------------------------------
/// 最常见的数据逃逸就是函数本地变量地址作为返回值
///     函数签名中使用引用类型的返回值，会报错，显示生命周期不够长
///     因为变量1在函数结束后生命结束被回收，其引用无法作为返回
/// fn f1() -> &i32 {&1} x
/// 
///     使用堆这种全局存储，然后返回指向堆的地址，就可以解决该问题
///     当然这个i32的例子，大可不必，直接用i32作为返回值即可，但换做其他更大的复杂结构体MyStruct
///     此时只创建一个MyStruct返回的时候不需要COPY一份。
/// fn f2() -> Box<i32> {Box::new(1)}
/// fn f3() -> Box<MyStruct> {Box::new(MyStruct{....}}
/// ----------------------------------------------------------------------------------
/// 堆用于嵌套结构体，下面代码说
fn main() {
    // 1 探究智能指针String
    // String的声明中能看出他里面就是个Vec<u8>，后者也是智能指针，所以其实是嵌套了智能指针
    // pub struct String {
    //     vec: Vec<u8>,
    // }
    // 但是他不是Vec的智能指针，而是str的
    // impl ops::Deref for String {
    //     type Target = str;
    // 
    //     #[inline]
    //     fn deref(&self) -> &str {
    //         unsafe { str::from_utf8_unchecked(&self.vec) }
    //     }
    // }
    
    // String是str的智能指针，所以可以调用str的方法 例如split是str的
    // 但不是Vec的智能指针，所以push等方法是不能调用的，毕竟直接调Vec<u8>的方法，对String来说基本没啥用
    let s = "123".to_string();
    s.split("2");

    // Box->String->str 两层智能指针， b也能调用str的方法
    let b = Box::new(s);
    b.split("2");

    // 上面现象是因为方法的接收参数是str的时候，则可以传入str String Box<String>，只要多次解引用能解出str类型即可
    // 同理&str 则可传入 &String &Box<String>，如下。
    test1(&b);
    fn test1(s: &str){
        s.split("2");
    }

    // 对于嵌套结构体，例如单链表，下面写法会报错说无法在编译器知道结构体的大小。显然在c语言中也是这样，结构体的体积需要能提前知道
    // struct Node { next: Node, val: i32 } 
    // c语言中一般通过指针类型解决，*Node 类型的next 指针是地址大小8字节(64位)，但是rust中*Node是裸指针，不能表示正常的类型，这个在pointer章节介绍
    // 如果用&Node那么他就是指针类型了，但是问题是&Node是引用，涉及到所有权，他没有所有权，一个结构体怎么能不持有成员呢
    // 用Box就可以完美替代引用了，因为Box是有所有权的地址
    struct Node {
        val: i32,
        next: Box<Node>,
    }

    // 不过上面还没处理末尾节点的next为空的问题，一般写法是将Node声明为enum有普通的节点和none节点两种
    #[derive(Debug)]
    enum Node2 {
        Some(i32, Box<Node2>), // 此时enum大小就是i32+一个指针的大小
        None
    }
    // 创建链表 3->2->1->none
    let null = Box::new(Node2::None);
    let n1 = Box::new(Node2::Some(1, null));
    let mut n = Box::new(Node2::Some(3, Box::new(Node2::Some(2, n1))));

    // null节点所有权moved到n1了，n1 moved到n了，此时无法再操作null和n1节点。而如果另一个n11也想next指向n1
    // 则n11和n都要有n1的所有权，此时引入多所有权的Rc（reference count）引用计数，这是Rc章节的内容
    


    // as_ref、as_mut方法，获取内容部分的引用&Node2
    let inner: &mut Node2 = n.as_mut();

    // 修改节点内部的值，利用enum的match匹配类型，val是&mut i32类型，用*val赋值
    match inner {
        Node2::Some(val, _) => *val = 33,
        _=> (),
    }
    println!("{:#?}", n); // 33->2->1->none


    // 单链表算法，找到倒数第k个值
    // 先实现几个通用方法
    impl Node2{
        fn is_none(&self) -> bool{
            match self {
                Node2::None => true,
                _ => false,
            }
        }
        fn val(&self) -> Option<i32> {
            match self {
                Node2::None => None,
                Node2::Some(val, _) => Some(*val),
            }
        }
        fn next(&self) -> Option<&Node2> {
            match self {
                Node2::None => None,
                Node2::Some(_, next) => Some(next.as_ref()),
            }
        }
    }

    fn lastK(root: &Node2, k: i32) -> Box<&Node2> {
        let mut r1 = root;
        let mut r2 = root;

        for i in 0..k {
            r1 = r1.next().expect("root len < k");
        }

        while !r1.is_none() {
            r1 = r1.next().unwrap();
            r2 = r2.next().unwrap();
        }

        Box::new(r2)
    }

    println!("{}", lastK(&n, 2).val().unwrap()); // 打印倒数第二个节点的值2

}

