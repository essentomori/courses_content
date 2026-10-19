use std::collections::BTreeMap;
use std::fmt::Display;
use std::io::{self, Write};

pub trait PrintResult {
    fn write_result(&self, output: &mut impl Write) -> io::Result<()>;
}

impl PrintResult for i32 {
    fn write_result(&self, output: &mut impl Write) -> io::Result<()> {
        writeln!(output, "{self}")
    }
}

impl PrintResult for str {
    fn write_result(&self, output: &mut impl Write) -> io::Result<()> {
        writeln!(output, "{self}")
    }
}

impl PrintResult for String {
    fn write_result(&self, output: &mut impl Write) -> io::Result<()> {
        self.as_str().write_result(output)
    }
}

impl<T: PrintResult + ?Sized> PrintResult for &T {
    fn write_result(&self, output: &mut impl Write) -> io::Result<()> {
        (*self).write_result(output)
    }
}

impl<T: Display> PrintResult for Vec<T> {
    fn write_result(&self, output: &mut impl Write) -> io::Result<()> {
        for value in self {
            writeln!(output, "{value}")?;
        }
        Ok(())
    }
}

impl<K: Display, V: Display> PrintResult for BTreeMap<K, V> {
    fn write_result(&self, output: &mut impl Write) -> io::Result<()> {
        for (key, value) in self {
            writeln!(output, "{key} = {value}")?;
        }
        Ok(())
    }
}

pub fn print_named_result<T: PrintResult>(name: &str, result: &T) {
    let mut output = io::stdout().lock();
    writeln!(output, "{name}").expect("Не удалось вывести имя функции");
    result
        .write_result(&mut output)
        .expect("Не удалось вывести результат");
}

#[macro_export]
macro_rules! print_result {
    ($name:expr, $expression:expr $(,)?) => {{
        let result = $expression;
        $crate::print_result::print_named_result($name, &result);
        result
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vectors_and_maps_are_printed_by_line() {
        let mut output = Vec::new();
        vec![1, 2].write_result(&mut output).unwrap();
        assert_eq!(String::from_utf8(output).unwrap(), "1\n2\n");

        let mut output = Vec::new();
        BTreeMap::from([("b", 2), ("a", 1)])
            .write_result(&mut output)
            .unwrap();
        assert_eq!(String::from_utf8(output).unwrap(), "a = 1\nb = 2\n");
    }

    #[test]
    fn scalar_and_string_are_printed_by_line() {
        let mut output = Vec::new();
        1_i32.write_result(&mut output).unwrap();
        "iu5".write_result(&mut output).unwrap();
        String::from("Rust").write_result(&mut output).unwrap();
        assert_eq!(String::from_utf8(output).unwrap(), "1\niu5\nRust\n");
    }

    #[test]
    fn macro_evaluates_once_and_returns_owned_result() {
        let mut calls = 0;
        let result = crate::print_result!("test", {
            calls += 1;
            vec![String::from("Rust")]
        });
        assert_eq!(calls, 1);
        assert_eq!(result, vec!["Rust"]);
    }
}
