use lab_rust_fp::field::field;
use serde_json::json;

fn main() {
    let goods = vec![
        json!({"title": "Ковер", "price": 2000, "color": "green"}),
        json!({"title": "Диван для отдыха", "color": "black"}),
        json!({"title": null, "price": null}),
    ];

    println!("Поле title:");
    for value in field(&goods, &["title"]).unwrap() {
        println!("{value}");
    }

    println!("Поля title и price:");
    for value in field(&goods, &["title", "price"]).unwrap() {
        println!("{value}");
    }

    if let Err(error) = field(&goods, &[]) {
        println!("Ошибка: {error}");
    };
}
