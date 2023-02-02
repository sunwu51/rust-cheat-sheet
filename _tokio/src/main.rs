// #[macro_use]
// use tokio::macros;

mod fs;
mod req;
mod sql;
mod web;
use std::{time::Duration, thread};

use futures::join;
use tokio::time::sleep;


// 1 这个宏可以给main添加async关键字
// 默认flavor是多线程的，线程数是核心数，也可以修改如下
#[tokio::main(worker_threads = 10)]
// #[tokio::main(flavor = "multi_thread", worker_threads = 10)]
// 下面是改成单线程
// #[tokio::main(flavor="current_thread")]
// #[tokio::main(flavor="current_thread")]
async fn main() {
    println!("串行");
    串行().await;
    println!("并行1");
    并行1().await;
    println!("并行2");
    并行2().await;
    println!("混用线程");
    混用线程().await;
    println!("混用线程2");
    混用线程2().await;
}

// 2 future是惰性的，await是阻塞当前协程的，所以是4s
async fn 串行() {
    let start = tokio::time::Instant::now();

    let a: i32 = f1().await;
    let b: i32 = f1().await;

    let end = tokio::time::Instant::now();

    println!("{}",  end.duration_since(start).as_millis()); // 2s
}

// 3 使用futures库的join，可以同时触发多个future，全部完成后返回，即并行
async fn 并行1() {
    let start = tokio::time::Instant::now();

    // a，b是f1和f1的返回值,i32类型
    let (a, b) = join!(f1(), f1());

    let end = tokio::time::Instant::now();

    println!("{}",  end.duration_since(start).as_millis()); // 2s
}

// 4 使用tokio::spawn(future)可以使future立即执行，并返回一个handle也实现了Future trait，
//  之前我们说future与js的promise最大区别是js的是立即执行，rust是惰性.这里handle就是立即执行了等价于promise
async fn 并行2() {
    let start = tokio::time::Instant::now();

    // JoinHandle类型实现了Future<Output=Result<T, JoinError>>
    let handle1 = tokio::spawn(async { f1().await; });
    let handle2 = tokio::spawn(async { f1().await; });

    // A await handle 本质还是spawn之后并发执行
    // handle1.await;
    // handle2.await;

    // B 当然也可以用futures::join!当然也是并发没有任何问题
    // let (r1, r2) = join!(handle1, handle2);

    // C tokio::join!与futures::join!作用一致，这样可以不引用futures库了
    let (r1, r2) = tokio::join!(handle1, handle2);

    let end = tokio::time::Instant::now();

    println!("{}",  end.duration_since(start).as_millis()); // 2s
}

// 5 不建议thread::spawn和tokio混用，如下
async fn 混用线程() {
    let start = tokio::time::Instant::now();
    let handle = tokio::spawn(f1());
    let t = thread::spawn(|| { thread::sleep(Duration::from_secs(2)) });
    t.join(); // 当前线程挂起去等2s来完成t
    handle.await; // 当前线程处理handle并等待其完成又是2s
    let end = tokio::time::Instant::now();

    println!("{}",  end.duration_since(start).as_millis()); // 4s
}

// 6 如果真的有cpu密集的计算，可以考虑spawn_blocking block_in_place两者都不会使用tokio的线程池
async fn 混用线程2() {
    let start = tokio::time::Instant::now();
    // spawn是使用基于线程的协程来执行任务
    let handle1 = tokio::spawn(f1());

    // spawn_blocking是开辟一个新的线程来立即执行
    let handle2 = tokio::task::spawn_blocking(|| thread::sleep(Duration::from_secs(2)));
    
    // block_in_place是阻塞当前线程，直到返回结构，res是返回的执行结果()
    let res = tokio::task::block_in_place(|| thread::sleep(Duration::from_secs(2)));
    
    handle1.await;
    handle2.await;
    
    // 因为协程的sleep不阻塞；新线程执行也是异步的；当前线程是阻塞的；
    // 三条线互不干扰并行执行。所以最后执行时间是2s
    let end = tokio::time::Instant::now();

    println!("{}",  end.duration_since(start).as_millis()); // 2s
}

async fn f1() -> i32{
    // tokio的sleep不阻塞线程
    sleep(Duration::from_secs(2)).await;
    
    // 不要在async内部用thread::sleep例如单线程时，会阻塞整个runtime
    // thread::sleep(Duration::from_secs(2));

    println!("f1");
    1
}


