fn main() {
    let mut v = vec![1, 2, 3, 4];

    // 基本操作，获取修改插入删除
    println!("{}", v[0]);
    v[0] = -1;
    // get操作可以防止越界，[i]不可
    let opt: Option<&i32> = v.get(0);
    v.push(5);
    v.remove(4);


    // 其他操作
    let r1: Option<i32> = v.pop();
    v.sort();
    v.append(&mut vec![6, 7, 8]);
    v.dedup(); // 去重
    let s: &[i32] = v.as_slice(); // 转为&[i32]切片
    
    // sub list的实现，或者就遍历然后append到空
    let x: &[i32] = &v.as_slice()[2..4];
    // as_slice和to_vec可以进行vec slice互转。
    let v2: Vec<i32> = x.to_vec();


    // 遍历
    for ele in v.iter() {
        println!("{}", ele);
    }
    // 另一种遍历
    for i in 0..v.len() {
        println!("{}", v[i]);
    }

    // 还有一种直接消费掉 vec的遍历
    for ele in v {
        println!("{}", ele);
    }


}
