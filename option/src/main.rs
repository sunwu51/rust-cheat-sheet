fn main() {
    let o1 = Some(1);
    let mut o2: Option<i32> = None;

    println!("{}", o1.is_some());
    println!("{}", o1.is_none());

    println!("{}", o1.unwrap());
    println!("{}", o1.expect("panic msg if none"));

    println!("{}", o2.get_or_insert(100));

    // as_ref转为内部是个引用，这样unwrap出来，就做到了对内容部分引用，该方法非常有用。
    let r: Option<&i32> = o1.as_ref(); // as_mut则是可变引用

    let i: &i32 = r.unwrap();
    println!("{:?} {:?}", o1, i); // 这里o1没有被改变


    // option -> result
    let res: Result<i32, &str> = o1.ok_or("err");

    // 列表相关的方法, map类型转换，filter做过滤
    let mut o3: Option<String> = o1
        .map(|x| format!("x={}", x))
        .filter(|s| s.starts_with("x="));

    // 迭代器

        // ele是&String类型
    for ele in o3.iter() {
        println!("{}", ele);    
    }

        // ele是&mut String
    for ele in o3.iter_mut() {
        ele.push_str("!!");
    }

        // ele是String类型，拥有所有权，也意味着o3被消费掉无法再被使用了。 o1不受影响
    for mut ele in o3.into_iter() {
        ele.push_str("??");
        println!("{}", ele);
    }



}
