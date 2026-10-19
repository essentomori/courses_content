use lab_rust_fp::sort::{sort_with_closure, sort_with_function};

fn main() {
    let data = vec![4, -30, 30, 100, -100, 123, 1, 0, -1, -4];
    println!("Исходный вектор: {data:?}");
    println!("С замыканием: {:?}", sort_with_closure(&data));
    println!("С функцией: {:?}", sort_with_function(&data));
}
