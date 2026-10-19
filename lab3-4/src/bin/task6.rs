use lab_rust_fp::cm_timer::{time_it, Timer};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    println!("Объект-таймер:");
    {
        let _timer = Timer::new();
        sleep(Duration::from_millis(500));
    }

    println!("Функция с замыканием:");
    let result = time_it(|| {
        sleep(Duration::from_millis(500));
        42
    });
    println!("Результат: {result}");
}
