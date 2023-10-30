use std::{collections::HashMap, hash::Hash};

pub fn group_by<'a, T, K, F>(it: impl Iterator<Item = &'a T>, f: F) -> HashMap<K, Vec<T>>
where
    K: Eq + Hash,
    F: Fn(&T) -> K,
    T: Clone + 'a
{
    let mut map: HashMap<K, Vec<T>> = HashMap::new();
    it.for_each(|v| {
        map.entry(f(v))
            .and_modify(|arr| arr.push(v.clone()))
            .or_insert_with(|| vec![v.clone()]);
    });
    map
}
