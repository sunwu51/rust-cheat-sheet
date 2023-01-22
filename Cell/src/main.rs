/// Cell改变内部可变性，还是单链表的例子中 Box可以修改节点的值，但是不能多所有者。Rc可以多所有者但是只能as_ref不能as_mut所以Rc下是不可变的
///     如果想在不可变变量下去改变可变性，即改变内部可变性，则需要用Cell或者RefCell，一般来说Cell适用于实现了Copy的例如u32等类型，对于其他类型一般使用RefCell更好
///     但是也可以对非Copy用Cell，下面的demo会看到
/// 我们先来说Cell，Cell顾名思义就是一个细胞一个盒子名称上类似Box或者Wrapper之类的，其实他做的事情就是Cell<T>包裹的数据t，通过cell提供的set方法可以直接修改t的值，而不需要受限于引用可变等规则
/// 注意Cell中的内容t是默认分配到栈上而不是堆，所以别想着那Cell直接替换Box或者Rc，Cell底层逻辑就是对原来的数据加了一层包装，用unsafe的代码强行获取可变的mut T来进行写操作
///     let x = Cell::new(1);
///     x.set(2);
/// 例如上面例子， x是一个不可变的变量，但是直接通过set方法就可以修改cell中的内容。如果对于非Copy类型，可以进行整体的结构体替换如下。
///     let x = Cell::new(User{id: 1, age: 10});
///     x.set(User{id: 1, age: 11});
///     println!("{:#?}", x.get()); // 只有实现了Copy才能get
/// Cell的关键就是整体替换，虽然Cell提供了get_mut方法，但是参数是&mut self也就是cell需要是可变的，那其实就不需要cell了
///     let mut = Cell::new(User{id: 1, age: 10})
///     x.get_mut().id = 2;
/// 或者用unsafe代码，as_ptr能获取底层裸指针，进行安全自负的写操作，但是不太建议这样写，这种情况可以考虑RefCell更好，尽量自己代码少些unsafe，对于裸指针也会有专门介绍
///     let x = Cell::new(User{id: 1, age: 10});
///     unsafe {
///         x.as_ptr().as_mut().unwrap().id = 2;
///     }
/// 
/// 

use std::{rc::Rc, cell::Cell};

/// 接下来我们会到单链表的例子，因为加了Cell就可以进行替换(写)操作了，所以之前Rc的版本中我们无法对val和next进行修改，这里我们对这两项都用Cell包装一下
// #[derive(Debug)] Debug无法派生，因为Cell中如果是非Copy的，那么就无法派生Debug
enum Node {
    Some(Cell<i32>, Cell<Rc<Node>>),
    None
}

#[derive(Debug,Clone, Copy)]
struct User {
    id: i32,
    age: i32,
}

fn main() {

    // -----------------上面提到的Cell的最基本的用法-----------------
    let x = Cell::new(User{id: 1, age: 10});
    x.set(User{id: 1, age: 11});
    println!("{:?}", x.get());
    unsafe {
        x.as_ptr().as_mut().unwrap().id = 2;
    }
    

    // -----------------单链表Cell加强后，使用Cell修改next来完成循环引用的壮举-----------------
    let n1 = Node::Some(
        Cell::new(1), 
        Cell::new(Rc::new(Node::None))
    );
    let n1_rc = Rc::new(n1);
    match n1_rc.as_ref() {
        Node::None => {},
        Node::Some(v, n) => {
            v.set(11);
            n.set(n1_rc.clone());   // n1的next重新指向n1，实现循环引用
        },
    }
    println!("rc count {}", Rc::strong_count(&n1_rc)); // 2
    // 然后打印多次next的值，发现全都是11，说明环已经形成。此时函数运行完成也无法清理节点的数据，这就是循环导致的内存泄漏
    //  当退出函数，n1_rc的引用数还是2，一个是自己，一个是clone。n1, n1_rc被清理，n1已经moved到后者了，所以只清理n1_rc
    //  n1_rc是个Rc，drop方法是计数-1，然后目前是2剪完了还是1，不为0，无法清理n1_rc和他指向的堆内存，引发内存泄漏。

    println!("{}", n1_rc.val());
    let mut x = n1_rc.next();
    for i in 0..10 {
        unsafe {
            println!("{}", x.as_ptr().as_mut().unwrap().val());
        }
        x = n1_rc.next();
    }
}


impl Node {
    fn next(&self) -> &Cell<Rc<Node>> {
        match &self {
            Node::None => panic!("no next"),
            Node::Some(n, v) => v
        }
    }

    fn val(&self) -> i32 {
        match &self {
            Node::None => panic!("no val"),
            Node::Some(n, _) => n.get()
        }
    }
}