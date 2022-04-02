use std::cmp::Ordering;

fn median(mut a: Vec<f32>) -> Option<f32> {
    // https://stackoverflow.com/questions/40408293/how-do-i-sort-nan-so-that-it-is-greater-than-any-other-number-and-equal-to-an
    a.sort_by(|&x, &y| match (x.is_nan(), y.is_nan()) {
        (true, true) => Ordering::Equal,
        (true, false) => Ordering::Greater,
        (false, true) => Ordering::Less,
        (false, false) => x.partial_cmp(&y).unwrap(),
    });
    let len = a.len();
    let half = len / 2;
    if len % 2 == 1 {
        // odd
        Some(a[half])
    } else if len == 0 {
        // empty
        None
    } else {
        // even
        let median = (a[half] + a[half - 1]) / 2.0;
        Some(median)
    }
}

fn main() {
    let answer = median(vec![1.0, 2.0, 5.0]);

    println!("median([1,2,5]) = {:?}", answer);
}

#[test]
fn empty_list() {
    let input = vec![];
    let expected_output = None;
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn sorted_list() {
    let input = vec![1.0, 4.0, 5.0];
    let expected_output = Some(4.0);
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn even_length() {
    let input = vec![1.0, 3.0, 5.0, 6.0];
    let expected_output = Some(4.0);
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn unsorted_list() {
    let input = vec![1.0, 5.0, 2.0];
    let expected_output = Some(2.0);
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}
