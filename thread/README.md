# thread
`spawn` `sleep`基础的用法，`move`将变量所有权转交到线程。

`mutex`互斥锁的使用，需要借助`Arc`和`clone`才能被多个线程同时持有其所有权。

`channel`介绍了最常用的`mpsc::channel`多发送，单接收，需要把tx clone给多个线程，channel本身维护了一个队列。

`Atomic`类型的使用，一般是`static`，这样就有`'static`的生命周期，即不需要move。

`Barrier`屏障，也需要`Arc`的帮助，wait方法会阻塞当前线程，直到阻塞的线程数达到设置的屏障值，就会重开所有屏障。rust中没有计数器，`Barrier`可以模拟出类似的功能。

`ThreadPool`在rust中没有内置，需要通过第三方的库引入并使用。
