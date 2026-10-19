use serde_json::{Map, Value};

pub fn field<'a>(
    items: &'a [Value],
    names: &[&str],
) -> Result<impl Iterator<Item = Value> + 'a, &'static str> {
    if names.is_empty() {
        return Err("Не указано ни одного поля");
    }

    let names: Vec<String> = names.iter().map(|name| name.to_string()).collect();

    Ok(items.iter().filter_map(move |item| {
        if names.len() == 1 {
            item.get(&names[0])
                .filter(|value| !value.is_null())
                .cloned()
        } else {
            let selected: Map<String, Value> = names
                .iter()
                .filter_map(|name| {
                    item.get(name)
                        .filter(|value| !value.is_null())
                        .map(|value| (name.clone(), value.clone()))
                })
                .collect();

            if selected.is_empty() {
                None
            } else {
                Some(Value::Object(selected))
            }
        }
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn single_field_skips_missing_and_null() {
        let items = vec![
            json!({"title": "Ковер"}),
            json!({"price": 2000}),
            json!({"title": null}),
            json!({"title": "Диван"}),
        ];
        let result: Vec<_> = field(&items, &["title"]).unwrap().collect();
        assert_eq!(result, vec![json!("Ковер"), json!("Диван")]);
    }

    #[test]
    fn multiple_fields_skip_empty_objects() {
        let items = vec![
            json!({"title": "Ковер", "price": 2000, "color": "green"}),
            json!({"title": "Диван", "price": null}),
            json!({"color": "black"}),
            json!({"title": null, "price": null}),
        ];
        let original = items.clone();
        let result: Vec<_> = field(&items, &["title", "price"]).unwrap().collect();
        assert_eq!(
            result,
            vec![
                json!({"title": "Ковер", "price": 2000}),
                json!({"title": "Диван"})
            ]
        );
        assert_eq!(items, original);
    }

    #[test]
    fn false_zero_and_empty_string_are_kept() {
        let items = vec![json!({"x": false}), json!({"x": 0}), json!({"x": ""})];
        let result: Vec<_> = field(&items, &["x"]).unwrap().collect();
        assert_eq!(result, vec![json!(false), json!(0), json!("")]);
    }

    #[test]
    fn empty_names_are_rejected() {
        assert!(field(&[], &[]).is_err());
    }
}
