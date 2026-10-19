use std::collections::HashSet;
use std::hash::Hash;

pub struct Unique<I, K, F> {
    iter: I,
    seen: HashSet<K>,
    key_fn: F,
}

impl<I> Unique<I, I::Item, fn(&I::Item) -> I::Item>
where
    I: Iterator,
    I::Item: Eq + Hash + Clone,
{
    pub fn new(items: impl IntoIterator<IntoIter = I>) -> Self {
        Self {
            iter: items.into_iter(),
            seen: HashSet::new(),
            key_fn: Clone::clone,
        }
    }
}

impl<I, K, F> Unique<I, K, F>
where
    I: Iterator,
    K: Eq + Hash,
    F: FnMut(&I::Item) -> K,
{
    pub fn with_key(items: impl IntoIterator<IntoIter = I>, key_fn: F) -> Self {
        Self {
            iter: items.into_iter(),
            seen: HashSet::new(),
            key_fn,
        }
    }
}

impl<I, K, F> Iterator for Unique<I, K, F>
where
    I: Iterator,
    K: Eq + Hash,
    F: FnMut(&I::Item) -> K,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .by_ref()
            .find(|item| self.seen.insert((self.key_fn)(item)))
    }
}

pub fn unique_strings<S>(items: S, ignore_case: bool) -> impl Iterator<Item = S::Item>
where
    S: IntoIterator,
    S::Item: AsRef<str>,
{
    Unique::with_key(items, move |value: &S::Item| {
        if ignore_case {
            value.as_ref().to_lowercase()
        } else {
            value.as_ref().to_owned()
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;

    #[test]
    fn numbers_keep_first_occurrence_order() {
        let result: Vec<_> = Unique::new(vec![3, 1, 3, 2, 1, 2]).collect();
        assert_eq!(result, vec![3, 1, 2]);
    }

    #[test]
    fn strings_respect_case_by_default() {
        let result: Vec<_> = Unique::new(["a", "A", "b", "B", "a"]).collect();
        assert_eq!(result, vec!["a", "A", "b", "B"]);
        assert_eq!(
            unique_strings(["a", "A", "a"], false).collect::<Vec<_>>(),
            vec!["a", "A"]
        );
    }

    #[test]
    fn case_insensitive_strings_keep_original_spelling() {
        let result: Vec<_> = unique_strings(["Ёж", "ёж", "Кот", "КОТ"], true).collect();
        assert_eq!(result, vec!["Ёж", "Кот"]);
    }

    #[test]
    fn source_is_consumed_lazily() {
        let consumed = Cell::new(0);
        let source = [1, 1, 2, 3]
            .into_iter()
            .inspect(|_| consumed.set(consumed.get() + 1));
        let mut result = Unique::new(source);
        assert_eq!(consumed.get(), 0);
        assert_eq!(result.next(), Some(1));
        assert_eq!(consumed.get(), 1);
        assert_eq!(result.next(), Some(2));
        assert_eq!(consumed.get(), 3);
    }
}
