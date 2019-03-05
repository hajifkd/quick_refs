use searches::*;

use rand::seq::SliceRandom;
use rand::{thread_rng, Rng as _};

#[test]
fn test_seq_search() {
    let n = 100;
    let mut rng = thread_rng();
    for _ in 0..100 {
        let mut vec: Vec<_> = (0..n).collect();
        vec.shuffle(&mut rng);
        vec.push(n + 1);
        let index = rng.gen_range(0, n);
        vec.swap(index, n);
        assert_eq!(seq_search(&vec[..], n + 1), index as isize);
    }
}

#[test]
fn test_bst_search() {
    let n = 1000;
    let mut rng = thread_rng();
    for _ in 0..100 {
        let mut vec: Vec<_> = (0..n).collect();
        for i in 1..10 {
            vec.push(n + i * 2);
        }
        vec.shuffle(&mut rng);

        for i in 1..10 {
            assert!(bst_search(&vec[..], n + i * 2));
            assert!(!bst_search(&vec[..], n + i * 2 + 1));
        }
    }
}
