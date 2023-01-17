use std::{collections::HashMap, thread};

/**
 * 这里对于读写和引用有个设计的技巧
 * 对于insert的时候，map需要拿到kv的所有权，保证数据被自己持有
 * 对于get的时候，则需要用引用类型，不应该把key给消耗掉
 */
fn main() {
    // HashMap需要指定泛型类型
    let mut m: HashMap<String, String> = HashMap::new();

    // 读 写
    for ele in vec![1,2,3,4,5,6] {
        let s = format!("{}", ele);
        m.insert(s.clone(), s.clone());
        println!("{}={}", s.clone(), m.get(&s).unwrap());
    }

    // 删除，并返回
    let res: Option<String> = m.remove("1");

    // 判断包含
    let contains: bool = m.contains_key("2");


    // 遍历
    for (k, v) in m.iter() {
        println!("{}={}", k, v);
    }

}
