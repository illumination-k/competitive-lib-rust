use cargo_snippet;

pub fn binary_search<T, F>(vec: &Vec<T>, key: T, is_ok: F) -> usize
    where 
        F: Fn(usize, T, &Vec<T>) -> bool,
        T: PartialOrd + Copy
{
    let mut ng = -1;
    let mut ok = vec.len() as isize;

    while (ok - ng).abs() > 1 {
        let mid = (ok + ng) / 2;

        if is_ok(mid as usize, key, vec) {
            ok = mid
        } else {
            ng = mid
        }
    }
    return ok as usize
}


#[cfg(test)]
mod test {
    use super::binary_search;

    #[test]
    fn is_ok_less() {
        let vec = vec![1, 14, 32, 51, 51, 51, 243, 419, 750, 910];

        fn is_ok(index: usize, key: usize, vec: &Vec<usize>) -> bool {
            if vec[index] >= key {
                true
            } else {
                false
            }
        }

        assert_eq!(binary_search(&vec, 51, is_ok), 3);
        assert_eq!(binary_search(&vec, 1, is_ok), 0);
        assert_eq!(binary_search(&vec, 910, is_ok), 9);
        assert_eq!(binary_search(&vec, 52, is_ok), 6);
        assert_eq!(binary_search(&vec, 0, is_ok), 0);
        assert_eq!(binary_search(&vec, 911, is_ok), 10)
    }
}