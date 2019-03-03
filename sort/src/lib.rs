pub fn insertion_sort(vs: &mut [impl PartialOrd]) {
    for i in 0..vs.len() {
        for j in 0..i {
            let i_targ = i - j;
            let i_cmp = i_targ - 1;

            if vs[i_cmp] > vs[i_targ] {
                vs.swap(i_cmp, i_targ)
            } else {
                break;
            }
        }
    }
}

pub fn heapify(vs: &mut [impl PartialOrd], i_targ: usize) {
    let i_left = i_targ * 2 + 1;
    let i_right = i_targ * 2 + 2;

    let mut i_largest = i_targ;

    if i_left < vs.len() && vs[i_targ] < vs[i_left] {
        i_largest = i_left;
    }

    if i_right < vs.len() && vs[i_largest] < vs[i_right] {
        i_largest = i_right;
    }

    if i_largest != i_targ {
        vs.swap(i_targ, i_largest);
        heapify(vs, i_largest);
    }
}

pub fn build_heap(vs: &mut [impl PartialOrd]) {
    for i in (0..vs.len() / 2).rev() {
        heapify(vs, i);
    }
}

pub fn heap_sort(vs: &mut [impl PartialOrd]) {
    build_heap(vs); // ヒープ構築はO(log n) + 2 O(log n / 2) + ... = O(n log n)

    for i in (0..=(vs.len() - 1)).rev() {
        vs.swap(0, i);
        heapify(&mut vs[0..i], 0); // ヒープ再構築は高々O(log n)
    }
}

pub fn quick_sort(vs: &mut [impl PartialOrd]) {
    let n = vs.len();

    if n <= 1 {
        return;
    }
    let mut i_less = 0;

    for i in 1..n {
        if vs[i_less] > vs[i] {
            for j in (i_less..i).rev() {
                vs.swap(j, j + 1);
            }
            i_less += 1;
        }
    }

    quick_sort(&mut vs[0..i_less]);
    quick_sort(&mut vs[(i_less + 1)..n]);
}

pub fn bucket_sort(vs: &mut [usize]) {
    let mut counts = vec![0; vs.iter().max().unwrap() + 1];

    for &v in vs.iter() {
        counts[v] += 1;
    }

    counts
        .into_iter()
        .enumerate()
        .flat_map(|(i, v)| std::iter::repeat(i).take(v))
        .enumerate()
        .for_each(|(i, v)| vs[i] = v);
}

pub fn marge_sort(vs: &mut [impl PartialOrd + Clone]) {
    let mut buf = vs.to_owned();

    fn recurr<T: PartialOrd + Clone>(result: &mut [T], buf: &mut [T]) {
        let n = result.len();

        if n <= 1 {
            return;
        }

        let n_half = n / 2;

        recurr(&mut buf[0..n_half], &mut result[0..n_half]);
        recurr(&mut buf[n_half..n], &mut result[n_half..n]);

        let mut i_right = n_half;

        for i in 0..n_half {
            while i_right < n && buf[i] > buf[i_right] {
                result[i + i_right - n_half] = buf[i_right].clone();
                i_right += 1;
            }
            result[i + i_right - n_half] = buf[i].clone();
        }

        for j in i_right..n {
            result[j] = buf[j].clone();
        }
    }

    recurr(vs, &mut buf);
}
