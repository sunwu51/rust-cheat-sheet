use std::{thread, time::Duration, sync::{atomic::{AtomicU32, Ordering}, Mutex, Arc, mpsc, Barrier}, rc::Rc};

use threadpool::ThreadPool;


// atomic类型，与java类似，可以进行多线程计数
static COUNT: AtomicU32 = AtomicU32::new(0);

fn main() {     
    thread_basic();
    channel_basic();

    // 线程池，需要用第三方库threadpool = "1.8.1"
    thread_pool_basic();
    
    barrier_basic();
}


fn thread_basic() { 
    // mutex锁，里面可以放任意资源和java的synchronized类似
    // mutex必须用Arc包裹，因为需要被多个线程持有所有权
    let arcm = Arc::new(Mutex::new(0));
    // spawn是产生并运行一个线程，sleep是休眠
    for i in 0..10 {

        let m = arcm.clone();

        // move关键字把闭包中捕捉的变量，所有权直接转让给线程
        // 因为线程不一定啥时候结束，可能当前上下文的i已经被清理了，所以不能用引用
        // 闭包相关的知识可以参考closure部分
        thread::spawn(move || {
            thread::sleep(Duration::from_secs(1));
            println!("hi i am a thread {}", i);
            {
                // lock是mutex的方法，这里自动解引用了，所以m可以直接调lock
                let mut content = m.lock().unwrap();
                println!("{}", content);
                *content = *content + 1;
                // mutex作用域就是锁的范围，所以要有个大括号
            }

            COUNT.fetch_add(1, Ordering::Relaxed)
        });
    }

    // rust线程是非守护的，主线程结束，进程退出
    thread::sleep(Duration::from_secs(5));

    println!("{}, {}", COUNT.fetch_add(0, Ordering::Relaxed), arcm.lock().unwrap());
}

fn channel_basic() {
    // 默认channel是异步的， 即send后就加到channel的队列中了， 代码继续往下运行
    // 直到rx从队列中有序的接收
    // channel中内容的类型可以自推断
    // channel也有sync_channel的，但是一般比较少用，同步版本队列中只能有0-1个数据，
    // 当有0个数据时recv阻塞等待有数据插入，当有1个数据时send会报错
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    let tx2 = tx.clone();
    thread::spawn(move ||{
        tx1.send(1).unwrap();
        println!("1 finish");
    });

    thread::spawn(move ||{
        tx2.send(2).unwrap();
        println!("2 finish");
    });

    thread::spawn(move ||{
        thread::sleep(Duration::from_secs(3));

        // 从队列中拿出所有的值，直到当前队列为空
        for i in rx {
            println!("{}", i);
        }
    });

     // rust线程是非守护的，主线程结束，进程退出
     thread::sleep(Duration::from_secs(5));

}

fn thread_pool_basic() {
    // 线程池，常驻线程有4个，可以多次使用，超过4个任务则等待，模型类似java的fixedThreadPool
    let n_workers = 4;
    let n_jobs = 8;
    let pool = ThreadPool::new(n_workers);

    let (tx, rx) = mpsc::channel();
    for _ in 0..n_jobs {
        let tx = tx.clone();
        pool.execute(move|| {
            tx.send(1).expect("channel will be there waiting for the pool");
        });
    }

    assert_eq!(rx.iter().take(n_jobs).fold(0, |a, b| a + b), 8);

}

fn barrier_basic() {
    let barrier = Arc::new(Barrier::new(10 + 1));
    let pool = ThreadPool::new(100);

    for _ in 0..10 {
        let b = barrier.clone();
        pool.execute(move || {
            println!("finish");
            thread::sleep(Duration::from_secs(2));
            b.wait();
        })
    }

    barrier.wait();
    println!("All finished");
}

