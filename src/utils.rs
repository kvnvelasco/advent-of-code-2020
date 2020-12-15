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
    let mut iterator = source.split(delimiter);

    iterator.collect()
}
