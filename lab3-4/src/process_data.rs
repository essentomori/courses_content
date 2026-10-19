use crate::field::field;
use crate::gen_random::gen_random;
use crate::unique::unique_strings;
use serde_json::Value;
use std::fs;
use std::path::Path;

pub fn load_data(path: &Path) -> Result<Vec<Value>, String> {
    let text = fs::read_to_string(path)
        .map_err(|error| format!("Не удалось прочитать {}: {error}", path.display()))?;
    let data: Vec<Value> = serde_json::from_str(&text)
        .map_err(|error| format!("Некорректный JSON в {}: {error}", path.display()))?;

    if data.iter().any(|item| !item.is_object()) {
        return Err("Ожидался массив JSON-объектов".to_owned());
    }

    Ok(data)
}

pub fn f1(data: &[Value]) -> Vec<String> {
    let names = field(data, &["job-name"])
        .expect("Имя поля задано")
        .filter_map(|value| value.as_str().map(str::to_owned));
    let mut result: Vec<_> = unique_strings(names, true).collect();
    result.sort_by_cached_key(|name| name.to_lowercase());
    result
}

pub fn f2(names: Vec<String>) -> Vec<String> {
    names
        .into_iter()
        .filter(|name| {
            name.to_lowercase()
                .strip_prefix("программист")
                .is_some_and(|rest| {
                    rest.chars()
                        .next()
                        .is_none_or(|ch| !ch.is_alphanumeric() && ch != '_')
                })
        })
        .collect()
}

pub fn f3(names: Vec<String>) -> Vec<String> {
    names
        .into_iter()
        .map(|name| format!("{name} с опытом Rust"))
        .collect()
}

pub fn f4(names: Vec<String>) -> Vec<String> {
    let salaries = gen_random(names.len(), 100_000, 200_000).expect("Границы заданы верно");
    names
        .into_iter()
        .zip(salaries)
        .map(|(name, salary)| format!("{name}, зарплата {salary} руб."))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn professions_are_unique_and_sorted_without_case() {
        let data = vec![
            json!({"job-name": "программист Rust"}),
            json!({"job-name": "Бухгалтер"}),
            json!({"job-name": "ПРОГРАММИСТ RUST"}),
            json!({"job-name": null}),
            json!({"job-name": 17}),
            json!({}),
        ];
        assert_eq!(f1(&data), vec!["Бухгалтер", "программист Rust"]);
    }

    #[test]
    fn programmer_filter_checks_first_word() {
        let names = [
            "Программист",
            "ПРОГРАММИСТ Rust",
            "Программист-разработчик",
            "Ведущий программист",
            "Программистка",
            "Программист123",
        ]
        .into_iter()
        .map(str::to_owned)
        .collect();
        assert_eq!(
            f2(names),
            vec!["Программист", "ПРОГРАММИСТ Rust", "Программист-разработчик"]
        );
    }

    #[test]
    fn experience_and_salary_are_added() {
        let names = f3(vec![
            "Программист C#".to_owned(),
            "Программист Rust".to_owned(),
        ]);
        assert_eq!(names[0], "Программист C# с опытом Rust");
        let result = f4(names.clone());
        assert_eq!(result.len(), names.len());
        for (line, name) in result.iter().zip(names) {
            let prefix = format!("{name}, зарплата ");
            let salary: i32 = line
                .strip_prefix(&prefix)
                .unwrap()
                .strip_suffix(" руб.")
                .unwrap()
                .parse()
                .unwrap();
            assert!((100_000..=200_000).contains(&salary));
        }
    }

    #[test]
    fn empty_pipeline_is_empty() {
        assert!(f4(f3(f2(f1(&[])))).is_empty());
    }
}
