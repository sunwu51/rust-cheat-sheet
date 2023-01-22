use std::{cell::{RefCell, RefMut, Cell}, rc::Rc};

/// 在Rc章节我们引出一个问题，就是多个强引用的情况下，如何进行数据的修改.Rc只实现了as_ref但是没有as_mut方法
/// 在Cell章节，我们给出了一个解决方案就是用Cell包裹在最外面，就可以通过Cell的set方法修改了，但是Cell一般用于Copy的数据类型
/// 比如Cell的get方法是只支持Copy的内部类型的get的
///     而RefCell则适用于引用或者说非Copy类型，他与Cell类似，只不过Cell包装了变量本身，给他做了set等利用裸指针的增强。
///     而RefCell则是包装了变量后，提供对变量的引用的租借，borrow_mut等方法获取可变引用进行数据修改。
/// 
/// RefCell 主要有两个方法 borrow 和 borrow_mut，返回一个对Cell中元素的借用但不是&XX而是专门的结构体
///     Ref<T>：borrow的返回，智能指针，可以当&T用
///     RefMut<T>：borrow_mut的返回，可以当&mut T用，最重要的是可以直接用 `*refmut.as = `来赋值
/// 
/// RefCell没有改变可变引用规则，仍是同时只存在多个不可变引用，或一个可变引用，只是编译期不检查
///     运行时，检查如果不符合规则，则直接panic
/// 
/// RefCell vs Cell：
///     Cell一节强调过，Cell的set是修改整个数据。
///     RefCell可以获取可变引用，既可以对引用整体修改，又可以获取结构体内的成员变量进行局部修改。
#[derive(Debug)]
enum Node {
    // 我们把val用Cell，next用RefCell<Rc<>>来包装，这样我们在可以修改val和next指向。
    // Cell适合包裹Copy，RefCell适合包裹非Copy，他们各自用在合适的场景
    Some(Cell<i32>,RefCell<Rc<Node>>),
    None
}

fn main() {
    // 首先构建 1->null的链式结构，这里我们对1节点用_t进行了暂存记录，用于后续打印他
    let null1 = wrapper(Node::None);
    let _t = Rc::new(Node::Some(Cell::new(1), null1));
    let n1 = RefCell::new(_t.clone());
    
    
    let _new = Rc::new(Node::Some(Cell::new(99), wrapper(Node::None))); 
    
    // 接下来利用RefCell的borrow_mut 得到RefMut，然后as_ref得到内部的数据&Node
    // 判断时得到Some中的val类型是&RefCell<Rc<i32>>，RefMut可以直接用*RefMut给其地址内的数据赋值
    match n1.borrow().as_ref() {
        Node::None => {},
        Node::Some(val, next) => {
            // 修改1号的val为100，并修改next指向一个新的链99->null
            // 最终成为100->99->null
            val.set(100);
            *next.borrow_mut() = _new;
        }
    }

    // 打印_t 100->99->null
    // 注意这里不能直接打印n1,因为RefCell类型不会展开里面的内容打印出来。
    println!("{:#?}", _t);

    
    let c = RefCell::new(5);

    // 同时可变借用和不可变借用是非法的，但是运行时才报错。
    // let borrowed_five = c.borrow_mut();
    // let borrowed_five2 = c.borrow();



    let node1 = Node::Some(Cell::new(1), wrapper(Node::None));
    let node1_rc = Rc::new(node1);
    let node1_rc_cell = RefCell::new(node1_rc.clone());

    // 将node1节点的next改为指向自己，形成死循环
    match node1_rc_cell.borrow_mut().as_ref() {
        Node::None => {},
        Node::Some(_, next)=>{
            *next.borrow_mut() = node1_rc.clone();
            // 用unsafe当然也可以，毕竟裸指针是万能的
            // unsafe {
            //     *next.as_ptr().as_mut().unwrap() = node1_rc.clone();
            // }
        }
    }
    // 死循环之后，用debug打印，就无限打印，内存爆炸
    // println!("{:#?}", node1_rc);

    // 打印Rc的引用计数次数，是只有3次，即上面有两次clone，弱引用为0，为什么没有报错呢
    // 这个死循环示意图： cell[rc[ val, $self ]]
    //      这里rc只被cell持有，并没有无限的引用指向，但循环是真实存在的。
    // Rc[A].clone->null 改为 Rc[A].clone->Rc[A].clone() 加上原来的Rc[A]和俩clone就是3个引用
    println!("{}", Rc::strong_count(&node1_rc)); // 3
    println!("{}", Rc::weak_count(&node1_rc));  // 0


    // 这么看下来死循环好像没什么问题，但是会引起作用域结束后的内存泄漏
    // 我们按照变量退出顺序， node1, node1_rc, node1_rc_cell都会进行清理
    // node1已经moved到node1_rc，也就是已经重新再堆上申请内存了。
    // node1_rc的drop是引用数-1，一共是3，这里减去1等于2
    // node1_rc_cell的持有node1_rc.clone，他的清理也会触发rc的drop-1，现在还剩1
    //  然后没了，到这里发现node1_rc的引用计数到最后没有清零，导致堆上的node数据内存泄漏了。

    // 如何解决内存泄漏 => 将Rc换成Weak就可以了，弱引用不需要计数
    //  这个链表场景就比较麻烦，不过对于像Person类型有parent和children互相引用的，就可以把
    //  其中一个作为Rc另一个是Weak。
    ()
}


fn wrapper<T>(t: T) -> RefCell<Rc<T>>{
    RefCell::new(Rc::new(t))
}

