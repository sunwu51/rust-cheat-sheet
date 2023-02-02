use std::fmt::Display;

use futures::{executor::block_on, join, Future, Stream, stream};

/// async、await介绍，不涉及异步底层框架。仅探讨rust语法
/// 
/// 
/// --- 需要引入官方futures包，这个包不内置，才支持该语法
/// 1 async修饰函数
async fn f1() -> i32 {
    println!("f1");
    1
}

/// 3 await用在async函数或代码块，阻塞f1立即去执行，执行完之后运行后续
/// 与block_on不同的是，await过程如果出现io，可以释放线程让其他协程使用，但block_on是完全阻塞线程
async fn f2() -> i32 {
    f1().await;
    println!("f2");
    2
}

fn main() {
    // 2 返回值类型f是Future，f不会立即去执行。只有poll或await时候才执行
    // 这里通过block_on方法阻塞其立即执行。
    let f = f2();
    block_on(f);
    block_on(async_main());
}

async fn async_main() {
    // 4 使用join!让多个future同时执行，并全部执行完成后返回
    join!(f1(), f2());
    f3().await;
    // println!("{}", f3().await);

}

// 5 async{}代码块，返回值也是Future。
//  async move{} 是常见的形式，move和闭包中move一致，都是转移所有权进去
fn f3() -> impl Future<Output=()> {
    let x = "1".to_owned();

    async move {
        println!("{x}");
    }
}

async fn f4() {
    stream::unfold
}
