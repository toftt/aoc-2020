use std::{collections::HashSet, fmt, hash::Hash, ops::Sub, str::FromStr};

#[allow(dead_code)]
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

#[allow(dead_code)]
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

#[derive(Clone)]
enum CharType {
    Digit,
    MinusSign,
    Other,
}
use CharType::{Digit, MinusSign, Other};

/// Find all separate integers in a string slice. If two runs of digits (optionally starting with -) is
/// separated by anything but an ascii digit the two runs will be
/// considered separate integers.
///
/// # Examples
///
/// ```
/// let ints = get_ints("mem[5315] = 56368");
/// assert_eq!(ints, vec![5315, 56368]);
/// ```
#[allow(dead_code)]
pub fn get_ints(s: &str) -> Vec<i64> {
    let mut result: Vec<i64> = Vec::new();
    let mut acc = String::new();
    let mut last_type = Other;

    for c in s.chars() {
        let current_type = match c {
            '-' => MinusSign,
            _ => match c.is_ascii_digit() {
                true => Digit,
                _ => Other,
            },
        };

        match (current_type.clone(), last_type) {
            (Digit, Other) | (Digit, Digit) => acc.push(c),
            (Digit, MinusSign) => {
                acc.push('-');
                acc.push(c)
            }
            (Other, Digit) | (MinusSign, Digit) => {
                result.push(acc.drain(..).collect::<String>().parse().unwrap())
            }
            _ => (),
        };
        last_type = current_type;
    }
    if acc.len() != 0 {
        result.push(acc.parse().unwrap());
    }

    result
}

#[test]
fn get_ints_tests() {
    assert_eq!(get_ints("hello 24 my name is 33"), vec![24, 33]);
    assert_eq!(get_ints("mem[5315] = 56368"), vec![5315, 56368]);
    assert_eq!(get_ints("1,20,8,12,0,14"), vec![1, 20, 8, 12, 0, 14]);
    assert_eq!(get_ints("-1,-20,8,12"), vec![-1, -20, 8, 12]);
    assert_eq!(get_ints("-1--3--4--5)"), vec![-1, -3, -4, -5]);
}

#[macro_export]
macro_rules! assign_ints {
    ($s:expr, $( $var:ident ),+) => {
        let mut ints_iter = crate::utils::get_ints($s).into_iter();

        $(
            #[allow(unused_variables)]
            let $var = ints_iter.next().unwrap();
        )+
    };
}

#[test]
fn assign_ints_test() {
    assign_ints!("x: 43, y: 14, z: -89", x, y, z);
    assert_eq!(x, 43);
    assert_eq!(y, 14);
    assert_eq!(z, -89);
}
