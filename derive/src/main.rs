use serde::{Serialize, Deserialize};
use serde_json::Value;

/// derive派生宏加到struct上
///     Debug是派生println!("{:?}")可以打印内部信息的
///     PartialEq和Eq是派生 == 运算符的，需要内部成员也实现
///     PartialOrd和Ord是 < > <= >= 运算符的，同样需要内部成员实现，大小的逻辑是
///         挨着比每个字段，如果第一个字段大，那就是大，一样的话就比第二个字段。
///     Clone是添加clone方法，可以深拷贝自身

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]

/// 序列化相关的，需要引入序列化的包，并开启derive特性
/// 此时只是可以序列化了，如何序列化，则还需要引入实现的包，例如json
/// serde = { version = "1", features = ["derive"] }
/// serde_json = "1.0"
#[derive(Serialize, Deserialize)]
struct User {
    id : i32,
    name: String,
    parent: Option<Box<User>>
}


fn main() {
    // None比Some小，所以u2大
    let u1 = User{id: 1, name: "z".to_string(), parent: None};
    let u2 = User{id: 1, name: "z".to_string(), parent: Some(Box::new(User{id: 2, name: "a".to_string(), parent: None}))};
    println!("u1=u2?{}", u1 == u2);
    println!("u1>u2?{}", u1 > u2);

    // 1比2小，所以u1大
    let u1 = User{id: 2, name: "a".to_string(), parent: None};
    let u2 = User{id: 1, name: "z".to_string(), parent: None};
    println!("u1=u2?{}", u1 == u2);
    println!("u1>u2?{}", u1 > u2);

    println!("{:?}", u1); // 打印详情单行
    println!("{:#?}", u1); // 多行


    // 结构体 -> json
    let json_str = serde_json::to_string(&u1).unwrap();
    
    // json -> 结构体 
    // 需要显示声明类型 ::<User> 或者等号左侧加User
    // 如果还没有准备好的类型，比如就想当jsonPath用，可以用Value类型，类似JsonObject
    let u3  = serde_json::from_str::<User>(&json_str).unwrap(); 
    
    println!("{}", json_str);
    println!("{:#?}", u3);

    // json!宏可以快速将一个json结构体，转为Value类型。上面的from_str，我们也可以显示的声明为Value类型。
    let v: Value = serde_json::json!({"a": {"b": [{"c":1}, {"c":2}]}}); // 这里面可以用null， null是serde_json中的
    // println!("{:#?}", v);
    let x = v["a"]["b"][1]["c"].as_i64().unwrap();
}
