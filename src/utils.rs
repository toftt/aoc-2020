use std::fmt::Debug;
use std::str::FromStr;

pub fn split<T, U>(s: &str, delimiter: &str) -> (T, U)
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
    U: FromStr,
    <U as FromStr>::Err: Debug,
{
    let mut parts = s.split(delimiter);
    (
        parts.next().unwrap().parse().unwrap(),
        parts.next().unwrap().parse().unwrap(),
    )
}
