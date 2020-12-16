use std::fmt::Debug;
use std::str::FromStr;

pub fn split_once_at(source: &'static str, pattern: &'static str) -> (&'static str, &'static str) {
    let index_of_first_occurance = source
        .find(pattern)
        .expect("Pattern does not exist in string");

    (
        &source[0..index_of_first_occurance],
        &source[index_of_first_occurance + pattern.len()..],
    )
}

pub fn split_into_array_by<'a>(source: &'a str, delimiter: &'static str) -> Vec<&'a str> {
    let iterator = source.split(delimiter);

    iterator.collect()
}

pub fn parse_input_into_vec<T>(input: &'static str) -> Vec<T>
where
    T: FromStr,
    T::Err: Debug,
{
    input.lines().map(|x| x.parse::<T>().unwrap()).collect()
}

pub fn parse_input_into_vec_str(input: &'static str) -> Vec<&'static str> {
    input.lines().collect()
}
