use std::collections::HashSet;

fn unique(a: Vec<i32>) -> Vec<i32> {
    if a.is_empty() {
        return vec![];
    }
    let first = *a.first().unwrap();
    let mut map = HashSet::from([first]);
    let mut out = vec![first];
    a.iter().for_each(|e| {
        if !map.contains(e) {
            map.insert(*e);
            out.push(*e);
        }
    });
    // Sort it only because the test cases expect so
    out.sort();
    out
}

// advanced 1: use generic types
// fn unique(a: Vec<T>) -> Vec<T> {
//     todo!();
// }

// advanced 2: keep items in order
// fn unique(a: Iterable<T>) -> Vec<T> {
//     todo!();
// }

// advanced 3: use iterators
// fn unique(a: Iterable<T>) -> Vec<T> {
//     todo!();
// }

fn main() {
    let input = vec![2, 1, 1];
    let answer = unique(input);
    println!("unique items -> {:?}", answer)
}


#[test]
fn empty_list() {
    let input = vec![];
    let expected_output = vec![];
    let actual_output = unique(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn sorted_list() {
    let input = vec![1, 4, 5];
    let expected_output = vec![1, 4, 5];
    let actual_output = unique(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn unsorted_list() {
    let input = vec![1, 5, 2];
    let expected_output = vec![1, 2, 5];
    let actual_output = unique(input);
    assert_eq!(actual_output, expected_output);
}


#[test]
fn unsorted_list_with_duplicates() {
    let input = vec![1, 5, 2, 2, 1];
    let expected_output = vec![1, 2, 5];
    let actual_output = unique(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn sorted_list_with_duplicates() {
    let mut input = vec![1, 5, 2, 2, 1];
    input.sort_by(|x,y| x.partial_cmp(y).unwrap());
    let expected_output = vec![1, 2, 5];
    let actual_output = unique(input);
    assert_eq!(actual_output, expected_output);
}
