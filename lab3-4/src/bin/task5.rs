use lab_rust_fp::print_result;
use std::collections::BTreeMap;

fn test_1() -> i32 {
    1
}

fn test_2() -> &'static str {
    "iu5"
}

fn test_3() -> BTreeMap<&'static str, i32> {
    BTreeMap::from([("a", 1), ("b", 2)])
}

fn test_4() -> Vec<i32> {
    vec![1, 2]
}

fn main() {
    print_result!("test_1", test_1());
    print_result!("test_2", test_2());
    print_result!("test_3", test_3());
    print_result!("test_4", test_4());
}
