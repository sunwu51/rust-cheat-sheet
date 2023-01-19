use std::{rc::Rc, sync::{Arc, Mutex}};

/// 写在前面，Rc与Box一样也是智能指针，也是在堆申请内存
/// 但是Box无法实现多所有权，当然目前为止还没有出现过多所有权
/// 不过Box中给了一个例子，就是有公共部分的两个单链表，无法被2个变量持有所有权
///     Rc提供了这样的功能，Rc(reference count)引用计数，是一些语言中用来gc的机制
///     Rc在Box基础上还存储了对象的引用个数，并且只有当引用的个数为0时，清理堆内存

/// 在只读场景下Rc可以完全替代Box
///     Rc不能像Box那样修改内部的数据，他只有as_ref方法，无as_mut方法
///     这也是显然的，Rc可以clone出多个指针，但是mut的只能有一个
/// 
/// 如果想要多所有权还想写操作，那需要用RefCell，会在RefCell章节展开
/// 
/// Rc是单线程的版本，Arc是其多线程版本。多线程共享数据就需要Arc了
#[derive(Debug)]
enum Node {
    Some(i32, Rc<Node>),
    None
}

fn main() {
    // n3 n4共用一段
    // 3--↓
    // 4->2->1->null
    let null = Rc::new(Node::None);
    let n1 = Rc::new(Node::Some(1, null.clone()));
    let n2 = Rc::new(Node::Some(2, n1.clone()));
    let n3 = Rc::new(Node::Some(3, n2.clone()));
    
    // Rc多持有者需要用clone方法，clone会增加一条强引用的计数
    let n4 = Rc::new(Node::Some(4, n2.clone()));

    // null:2, n1:2, n2:3, n3:1, n4:1
    // 通过静态方法Rc::strong_count打印引用次数，注意公共部分为2
    println!("null:{}, n1:{}, n2:{}, n3:{}, n4:{}", 
        Rc::strong_count(&null),
        Rc::strong_count(&n1),
        Rc::strong_count(&n2),
        Rc::strong_count(&n3),
        Rc::strong_count(&n4),
    );

    // as_ref与Box中一致 可以返回&T，但是他没有as_mut方法
    // 这样符合多个读引用 或 一个写引的设定
    // 同时这个保障，可以避免产生循环，因为循环需要尾部再指回头，就需要写操作
    println!("{}", n1.as_ref().val().unwrap());

    // 当然RefCell<Rc<T>>的组合可以进行写操作，于是会产生循环这在RefCell中展开


    // Arc是多线程版本的Rc，多线程共享内存，用同一个数据很正常
    //  例如Mutex，Barrier等，要在线程内使用，就必须用Arc
    //      通过clone，move到线程里面，当然Arc也不支持as_mut
    //      在thread章节有Arc<Mutex>>的demo
    let n = Arc::new(Mutex::new(0));
}

// 同样把val和next相关的方法从Box拷过来
impl Node {
    fn is_none(&self) -> bool{
        match self {
            Node::None => true,
            _ => false,
        }
    }
    fn val(&self) -> Option<i32> {
        match self {
            Node::None => None,
            Node::Some(val, _) => Some(*val),
        }
    }
    fn next(&self) -> Option<&Node> {
        match self {
            Node::None => None,
            Node::Some(_, next) => Some(next.as_ref()),
        }
    }
}