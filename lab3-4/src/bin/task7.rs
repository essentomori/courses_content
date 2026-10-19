use lab_rust_fp::cm_timer::Timer;
use lab_rust_fp::print_result;
use lab_rust_fp::process_data::{f1, f2, f3, f4, load_data};
use std::env;
use std::path::Path;
use std::process::ExitCode;

fn run() -> Result<(), String> {
    let mut args = env::args_os().skip(1);
    let path = args
        .next()
        .ok_or("Укажите путь: cargo run --bin task7 -- data/data_light.json")?;
    if args.next().is_some() {
        return Err("Ожидался один аргумент: путь к JSON-файлу".to_owned());
    }

    let data = load_data(Path::new(&path))?;
    let _timer = Timer::new();
    let names = print_result!("f1", f1(&data));
    let programmers = print_result!("f2", f2(names));
    let experienced = print_result!("f3", f3(programmers));
    print_result!("f4", f4(experienced));
    Ok(())
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("Ошибка: {error}");
            ExitCode::FAILURE
        }
    }
}
