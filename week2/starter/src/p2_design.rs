//! In this file, you will design functions to implement a high-level specification.
//! The goal is to have you explore the different possible implementations of a spec in Rust,
//! and to articulate the trade-offs in terms of generality, performance, and usability.

// EXAMPLE: below is a completed function that demonstrates how each problem will work.
// Each problem contains a specification above the function. Your goal is to design the function
// signature and implementation. For each parameter and the return type, you should describe
// (a) a reasonable space of possible types, and (b) your rationale for picking a particular type.
// Make sure to add at least one unit test for your particular implementation.

/// round_all is a function that takes:
///   * v: representing a collection of numbers
/// and rounds every number in-place in v to the nearest integer.
pub fn round_all(
    // (1) v could be a Vec<_>, &Vec<_>, &mut Vec<_>, &[_], or &mut[_]. I choose &mut[_] because
    //     we do not need to change the size or order of the collection, but do need to change the elements.
    // (2) v could be a &mut [{number type}], and "round to the nearest integer" implies the use of floats.
    // (3) The choice of f32 vs. f64 is arbitrary -- we would need to use more advanced features to write one
    //     function that works for both types, so we arbitrarily pick f32 for now.
    v: &mut [f32],
)
// No return value, since this function only mutates an input.
{
    for n in v.iter_mut() {
        *n = n.round();
    }
}

#[test]
fn round_all_test() {
    let mut v = vec![0.3, 0.7];
    round_all(&mut v);
    assert_eq!(v, vec![0., 1.]);
}

// Now you try!

/// P2a: find_contains is a function that takes:
///   * haystack: representing a collection of strings
///   * needle: representing a particular string
/// and returns a value:
///   * representing which strings in the collection contain the needle
pub fn find_contains(haystack: &[&str], needle: &str) -> Vec<bool> {
    let mut contains: Vec<bool> = Vec::with_capacity(haystack.len());
    for s in haystack.iter() {
        contains.push(s.contains(needle));
    }
    return contains;
}

#[test]
fn find_contains_test() {
    let v = vec!["a", "abc", "c"];
    let contains = find_contains(v.as_slice(), "a");
    assert_eq!(contains, vec![true, true, false]);
}

const PROGRESS_BAR_LENGTH: usize = 10;

/// P2b: fill_progress_bar is a function that takes:
///   * buf: a string to fill
///   * delims: a pair of delimiters to wrap the bar
///   * frac: the fraction of the bar to display
/// Then places a textual representation of the progress bar into `buf`.
/// For example, at a progress of 20% with bracketed delimiters, the bar would be:
///   [==        ]
pub fn fill_progress_bar(buf: &mut String, delims: (char, char), frac: f64) {
    buf.clear();
    let (delims_start, delims_end) = delims;
    buf.push(delims_start);
    let n_equals = (frac * PROGRESS_BAR_LENGTH as f64).round() as usize;
    for _ in 0..n_equals {
        buf.push('=');
    }
    for _ in n_equals..PROGRESS_BAR_LENGTH {
        buf.push(' ');
    }
    buf.push(delims_end);
}

#[test]
fn test_fill_progress_bar() {
    let mut s = String::new();
    fill_progress_bar(&mut s, ('[', ']'), 0.5);
    assert_eq!(s, String::from("[=====     ]"));

    fill_progress_bar(&mut s, ('<', '>'), 1.0);
    assert_eq!(s, String::from("<==========>"));
    fill_progress_bar(&mut s, ('{', '}'), 0.0);
    assert_eq!(s, String::from("{          }"));
}
