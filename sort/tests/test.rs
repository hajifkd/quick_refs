extern crate rand;

use rand::seq::SliceRandom;
use rand::thread_rng;
use sort::*;

fn test_sort(f: &Fn(&mut [usize])) {
    for _ in 0..100 {
        let n = 100;
        let mut rng = thread_rng();
        let mut vec: Vec<_> = (0..n).collect();
        vec.shuffle(&mut rng);
        //assert_eq!(vec, (0..n).collect::<Vec<_>>());
        f(&mut vec[..]);
        assert_eq!(vec, (0..n).collect::<Vec<_>>());
    }
}

#[test]
fn test_insertion_sort() {
    test_sort(&insertion_sort);
}

#[test]
fn test_heap_sort() {
    test_sort(&heap_sort);
}

#[test]
fn test_quick_sort() {
    test_sort(&quick_sort);
}

#[test]
fn test_bucket_sort() {
    test_sort(&bucket_sort);
}

#[test]
fn test_marge_sort() {
    test_sort(&marge_sort);
}
