use lab_rust_fp::gen_random::gen_random;
use lab_rust_fp::unique::{unique_strings, Unique};

fn main() {
    let values = vec![1, 1, 1, 2, 2, 3, 1];
    println!(
        "Без повторений: {:?}",
        Unique::new(values).collect::<Vec<_>>()
    );

    let words = ["a", "A", "b", "B", "a", "A"];
    println!(
        "С учётом регистра: {:?}",
        Unique::new(words).collect::<Vec<_>>()
    );
    println!(
        "Без учёта регистра: {:?}",
        unique_strings(words, true).collect::<Vec<_>>()
    );

    let numbers = gen_random(10, 1, 3).unwrap();
    println!(
        "Из генератора: {:?}",
        Unique::new(numbers).collect::<Vec<_>>()
    );
}
