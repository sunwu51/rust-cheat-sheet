use std::{slice::Iter, vec::IntoIter};

fn main() {
    // 很多包下都有Iter，不像java是一个统一的接口
    let v = vec!["1".to_string(), "2".to_string()];
    let it: Iter<String> = v.iter();
    let it2: IntoIter<String> = v.clone().into_iter();

    // 遍历, ele是&String，不获取所有权的
    for ele in it {
        println!("{}", ele);
    }

    // IntoIter类型遍历是String类型，获取所有权
    for ele in it2 {
        println!("{}", ele);
    }


    option_basic();
}


fn option_basic() {
    let v = vec!["1".to_string(), "2".to_string()];
    let it: Iter<String> = v.iter();

    // iter在执行各种遍历的操作 map flatmap等，都是消耗自身的，因为遍历完了it就没用了
    let map_res = it.map(|s|s.parse::<i32>().unwrap());

    let filter_res = map_res.filter(|i| *i < 10);

    // flat_res此时类型是FlatMap<Filter<Map<Iter<String>>>，看上去很吓人其实不用管，因为他肯定是impl Iterator
    let flat_res = filter_res.flat_map(|i| vec![i, i *i]);

    let sum = flat_res.reduce(|a, b| a + b);


    // 用fold代替reduce，reduce, 第一个参数是初值
    // flat_res.fold(0, |a, b| a + b);
}