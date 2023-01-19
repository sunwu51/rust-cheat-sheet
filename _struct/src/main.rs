
// 结构体的基本声明方式
#[derive(Debug)]
struct Node {
    id: i32,
    // 字段用逗号而不是分号隔开，字符串用String来保持所有权，不要用&str
    // 因为结构体是内部成员的所有权持有者
    name: String,
}

// 结构体实现方法
impl Node {
    // 静态方法，可以不指定self，通过Node::new调用
    fn new(id: i32) -> Node {
        Node { id, name: String::default()}
    }

    // 不消耗自身的方法，通过n.to_string()调用，等价于Node::to_string(&n)
    fn to_string(&self) -> String {
        let name: &String = &self.name;
        let id: i32 = self.id;
        format!("id: {}, name: {}", id, name)
    }

    // 消耗掉自身的方法，通过n.get_name或者Node::get_name(n)调用，调用完后，结构体被moved
    fn get_name(self) -> String {
        self.name
    }
}

fn main() {
    // 结构体的创建，需要指定所有字段的值来创建
    let n = Node { id: 1, name: "foo".to_string() };
    // 或者自己创建new方法来创建
    let n = Node::new(1);


    // 如果将内部属性所有权moved，会导致struct整个moved
    let s: String = n.name;
    // 此时再访问n，会报错，因为n已经被消耗了。 println!("{:#?}", n);

    // &n.name在这等于&(n.name)，对成员进行引用后，就不会导致消耗掉结构体本身
    let n = Node::new(1);
    let s: &String = &n.name;
    println!("{:#?}", n);

    // 通过可变引用引用出结构体字段，并对其进行修改
    let mut n = Node::new(1);
    let s = &mut n.name;
    s.push_str("frank");
    println!("{:#?}", n);

    // 调用to_string方法
    println!("{:#?}", n.to_string());
    println!("{:#?}", n.get_name());


    // 上面 &n.name 是 &(n.name) 还是 (&n).name是有歧义的
    // 事实上结果是前者，这也在&self.name那里同样出现，该如何理解呢
    // 因为(&n).name是非法的，其返回类型是String，拥有权，但本身是引用不可能通过引用还拿到了内部成员所有权
    // 因而只有&(n.name)是合理的解释



    let p = Pair(1,2);
    let one = p.0;
    let two = p.1;
}


// 元组型结构体，不需要指定字段的名字，只写类型即可，通过
struct Pair(i32, i32);