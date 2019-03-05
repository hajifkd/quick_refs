pub fn seq_search<T: PartialEq>(vs: &[T], v: T) -> isize {
    if let Some(i) = vs
        .iter()
        .enumerate()
        .filter(|&(_, v1)| *v1 == v)
        .map(|(i, _)| i)
        .next()
    {
        i as _
    } else {
        -1
    }
}

pub struct AVLNode<T: PartialOrd + PartialEq + Copy> {
    height: isize,
    value: T,
    left: Option<Box<AVLNode<T>>>,
    right: Option<Box<AVLNode<T>>>,
}

impl<T: PartialOrd + PartialEq + Copy> AVLNode<T> {
    fn new_leaf(v: T) -> Box<Self> {
        Box::new(AVLNode {
            height: 0,
            left: None,
            right: None,
            value: v,
        })
    }

    pub fn build_tree(vs: &[T]) -> Box<Self> {
        vs[1..]
            .iter()
            .fold(Self::new_leaf(vs[0]), |x, &acc| append(x, acc))
    }

    fn left_height(&self) -> isize {
        if let Some(ref left) = self.left {
            left.height
        } else {
            -1
        }
    }

    fn right_height(&self) -> isize {
        if let Some(ref right) = self.right {
            right.height
        } else {
            -1
        }
    }

    fn update_height(&mut self) {
        self.height = std::cmp::max(self.left_height(), self.right_height()) + 1;
    }
}

fn rotate_right<T: PartialOrd + PartialEq + Copy>(
    mut root: Box<AVLNode<T>>,
) -> Box<AVLNode<T>> {
    let mut right = root.right.take().unwrap();

    if right.right_height() >= right.left_height() {
        // 右-右
        let right_left = right.left.take();
        root.right = right_left;
        root.update_height();
        right.left = Some(root);
        root = right;
    } else {
        // 右-左
        let mut right_left = right.left.take().unwrap(); // この時点でleftはNoneではない
        let right_left_left = right_left.left.take();
        let right_left_right = right_left.right.take();
        root.right = right_left_left;
        root.update_height();
        right_left.left = Some(root);
        right.left = right_left_right;
        right.update_height();
        right_left.right = Some(right);
        root = right_left;
    }

    root
}

fn rotate_left<T: PartialOrd + PartialEq + Copy>(
    mut root: Box<AVLNode<T>>,
) -> Box<AVLNode<T>> {
    let mut left = root.left.take().unwrap();

    if left.left_height() >= left.right_height() {
        // 左-左
        let left_right = left.right.take();
        root.left = left_right;
        root.update_height();
        left.right = Some(root);
        root = left;
    } else {
        // 左-右
        let mut left_right = left.right.take().unwrap();
        let left_right_left = left_right.left.take();
        let left_right_right = left_right.right.take();
        left.right = left_right_left;
        left.update_height();
        left_right.left = Some(left);
        root.left = left_right_right;
        root.update_height();
        left_right.right = Some(root);
        root = left_right;
    }

    root
}

pub fn append<T: PartialOrd + PartialEq + Copy>(
    mut root: Box<AVLNode<T>>,
    v: T,
) -> Box<AVLNode<T>> {
    if v >= root.value {
        if root.right.is_none() {
            root.right = Option::Some(AVLNode::new_leaf(v));
        } else {
            let right = root.right.take().unwrap();
            root.right = Option::Some(append(right, v));
        }

        if root.right_height() > root.left_height() + 1 {
            root = rotate_right(root);
        }
    } else {
        if root.left.is_none() {
            root.left = Option::Some(AVLNode::new_leaf(v));
        } else {
            let left = root.left.take().unwrap();
            root.left = Option::Some(append(left, v));
        }

        if root.left_height() > root.right_height() + 1 {
            root = rotate_left(root);
        }
    }

    root.update_height();

    root
}

fn remove_rightest_value<T: PartialOrd + PartialEq + Copy>(
    mut root: Box<AVLNode<T>>,
) -> (Option<Box<AVLNode<T>>>, T) {
    if root.right.is_none() {
        (root.left.take(), root.value)
    } else {
        let (left, value) = remove_rightest_value(root.right.take().unwrap());
        root.right = left;

        if root.left_height() > root.right_height() + 1 {
            root = rotate_left(root);
        }

        root.update_height();
        (Some(root), value)
    }
}

pub fn delete<T: PartialOrd + PartialEq + Copy>(
    mut root: Box<AVLNode<T>>,
    v: T,
) -> (bool, Option<Box<AVLNode<T>>>) {
    let mut result = true;
    if v == root.value {
        if let Some(left) = root.left.take() {
            let (left_new, value) = remove_rightest_value(left);
            root.value = value;
            root.left = left_new;

            if root.right_height() > root.left_height() + 1 {
                root = rotate_right(root);
            }
        } else {
            return (true, root.right.take());
        }
    } else if v > root.value {
        if let Some(right) = root.right.take() {
            let (d1, d2) = delete(right, v);
            result = d1;
            root.right = d2;

            if root.left_height() > root.right_height() + 1 {
                root = rotate_left(root);
            }
        } else {
            return (false, Some(root));
        }
    } else {
        if let Some(left) = root.left.take() {
            let (d1, d2) = delete(left, v);
            result = d1;
            root.left = d2;

            if root.right_height() > root.left_height() + 1 {
                root = rotate_right(root);
            }
        } else {
            return (false, Some(root));
        }
    }

    root.update_height();
    (result, Some(root))
}

pub fn bst_search<T: PartialOrd + PartialEq + Copy>(vs: &[T], v: T) -> bool {
    let tree = AVLNode::build_tree(vs);

    fn search_tree<T: PartialOrd + PartialEq + Copy>(t: &Box<AVLNode<T>>, v: T) -> bool {
        if v == t.value {
            true
        } else if v > t.value {
            t.right.as_ref().map(|r| search_tree(r, v)).unwrap_or(false)
        } else {
            t.left.as_ref().map(|l| search_tree(l, v)).unwrap_or(false)
        }
    }

    search_tree(&tree, v)
}

#[cfg(test)]
mod test {
    use super::*;

    use rand::seq::SliceRandom;
    use rand::{thread_rng, Rng as _};

    fn check_mod_tree_rule(t: &Box<AVLNode<isize>>, max: Option<isize>, min: Option<isize>) {
        assert!(max.map(|s| s >= t.value).unwrap_or(true));
        assert!(min.map(|s| s <= t.value).unwrap_or(true));

        if let Some(ref left) = t.left {
            check_mod_tree_rule(left, Some(t.value), min);
        }

        if let Some(ref right) = t.right {
            check_mod_tree_rule(right, max, Some(t.value));
        }
    }

    fn check_height_diff(t: &Box<AVLNode<isize>>) -> isize {
        let height_left = t
            .left
            .as_ref()
            .map(|left| check_height_diff(left))
            .unwrap_or(-1);
        let height_right = t
            .right
            .as_ref()
            .map(|right| check_height_diff(right))
            .unwrap_or(-1);

        assert!((height_left - height_right).abs() <= 1);

        std::cmp::max(height_left, height_right)
    }

    #[test]
    fn test_building_avl_tree() {
        let mut rng = thread_rng();
        for _ in 0..100 {
            let vec: Vec<_> = (0..1000).map(|_| rng.gen_range(0, 1000)).collect();

            let tree = AVLNode::build_tree(&vec[..]);

            check_mod_tree_rule(&tree, None, None);
            check_height_diff(&tree);
        }
    }

    #[test]
    fn test_deleting_elements_from_avl_tree() {
        let mut rng = thread_rng();
        for _ in 0..100 {
            let mut vec: Vec<_> = (0..1000).map(|_| rng.gen_range(0, 1000)).collect();

            for i in 10001..10020 {
                vec.push(i);
            }

            vec.shuffle(&mut rng);

            let mut tree = AVLNode::build_tree(&vec[..]);

            check_mod_tree_rule(&tree, None, None);
            check_height_diff(&tree);

            for i in 10001..10020 {
                // 1st try (must be found)
                let (found, t) = delete(tree, i);
                assert!(found);
                tree = t.unwrap();
                check_mod_tree_rule(&tree, None, None);
                check_height_diff(&tree);

                // 2nd try (must fail)
                let (found, t) = delete(tree, i);
                assert!(!found);
                tree = t.unwrap();
                check_mod_tree_rule(&tree, None, None);
                check_height_diff(&tree);
            }
        }
    }
}
