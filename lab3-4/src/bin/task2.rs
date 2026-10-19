use lab_rust_fp::gen_random::gen_random;

fn main() {
    let values: Vec<_> = gen_random(5, 1, 3).unwrap().collect();
    println!("Пять чисел от 1 до 3: {values:?}");

    let values: Vec<_> = gen_random(4, 7, 7).unwrap().collect();
    println!("Одинаковые границы: {values:?}");

    let values: Vec<_> = gen_random(0, 1, 3).unwrap().collect();
    println!("Ноль элементов: {values:?}");

    if let Err(error) = gen_random(5, 3, 1) {
        println!("Ошибка: {error}");
    }
}
