use std::{collections::HashSet, fmt, hash::Hash, ops::Sub, str::FromStr};

pub fn split<T, U>(s: &str, delimiter: &str) -> (T, U)
where
    T: FromStr,
    <T as FromStr>::Err: fmt::Debug,
    U: FromStr,
    <U as FromStr>::Err: fmt::Debug,
{
    let mut parts = s.split(delimiter);
    (
        parts.next().unwrap().parse().unwrap(),
        parts.next().unwrap().parse().unwrap(),
    )
}

pub fn count_pairs_of_sum<T>(slice: &[T], sum: T) -> i32
where
    T: Eq + Sub<Output = T> + Hash + Copy,
{
    let mut set: HashSet<T> = HashSet::new();
    let mut count = 0;

    for num in slice {
        if set.get(&num).is_some() {
            count += 1;
        }

        set.insert(sum - *num);
    }

    count
}
