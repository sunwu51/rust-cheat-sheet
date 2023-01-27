/// 1 先说数组，[T;n]这是数组的类型签名，n的长度不同，代表的是不同的类型，所以一般较少使用数组类型。
/// 更多的是使用切片slice，切片类型名为[T]，经常作为入参是引用类型即&[T]。因为一般无法直接操作切片[T]都是对其引用&[T]操作
///     切片内存储的是指向数组的指针长度和容量，容量大于等于长度
/// str是常见的一种切片

fn main() {
    let mut arr1: [i32; 5] = [1, 2, 3, 4, 5];

    // 数组转切片
    let sli1: &[i32] = arr1.as_slice();
    let mut sli1: &mut [i32] = arr1.as_mut_slice();

    // 切片截取
    let sli2: &mut [i32] = &mut sli1[1..3];
    // 修改某一项的值
    sli2[0] = 100;
    println!("{:?}",  sli2);

    // 与golang中不同，rust中的切片只是一个view，无法对其进行append，虽然能改

    /*
        in Go, a slice is a proxy to another container which allows both observing and mutating the container,
        in Rust, a slice is a view in another container which therefore only allows observing the container (though it may allow mutating the individual elements). 
    */
}
