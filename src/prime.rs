use cargo_snippet::snippet;
use num_traits::{ PrimInt, NumAssign, zero, one, NumCast};

#[snippet("is_prime")]
pub fn is_prime<T: PrimInt+NumAssign>(n: T) -> bool {
    let mut flag: bool = true;

    if n == one() { flag = false }
    let mut i: T = one::<T>().signed_shl(1);
    while i * i <= n {
        if n % i == zero() {
            flag = false;
            break;
        }
        i += one();
    }
    flag
}

#[snippet("enum_divisors")]
pub fn enum_divisors<T: PrimInt+NumAssign>(n: T) -> Vec<T> {
    let mut res: Vec<T> = Vec::new();

    let mut i: T = one();

    while i * i <= n {
        if n % i == zero() {
            res.push(i);
            if n / i != i { res.push(n / i) }
        }
        i += one();
    }
    res.sort();
    res
}


#[snippet("prime_factorize")]
pub fn prime_factorize<T: PrimInt+NumAssign>(mut n: T) -> Vec<(T, T)> {
    let mut res: Vec<(T, T)> = Vec::new();

    let mut i: T = one::<T>().signed_shl(1);

    while i * i <= n {
        if n % i == zero() {
            let mut ex = zero::<T>();

            while n % i == zero() {
                ex += one();
                n = n / i;
            }
            res.push((i, ex));
        }
        i += one();
    }

    if n != one() {res.push((n, one()))}

    res
}


#[snippet("sieve_of_eratosthenes")]
pub fn sieve_of_eratosthenes<T: NumCast>(n: T) -> Vec<usize> {
    let n = match n.to_usize() {
        Some(x) => x,
        None => panic!()
    };
    let mut primes = vec![];
    let mut list_dequeue: std::collections::VecDeque<usize> = (2..=n).collect();
    
    if n < 2 { return primes }
    if n == 2 { return vec![2] }

    while list_dequeue.front().unwrap().pow(2) <= n {
        let first = list_dequeue.pop_front().unwrap();
        primes.push(first);

        list_dequeue = list_dequeue.into_iter().filter(|&x| x % first != 0).collect();
    }
    let mut list_vec = list_dequeue.into_iter().collect::<Vec<usize>>();
    primes.append(&mut list_vec);
    primes
}


#[cfg(test)]
mod test {
    use super::is_prime;
    use super::enum_divisors;
    use super::prime_factorize;
    use super::sieve_of_eratosthenes;

    #[test]
    fn test_is_prime_2() {
        let flag = is_prime(2);

        assert_eq!(flag, true);
    }

    #[test]
    fn test_is_prime_13() {
        let flag = is_prime(13);
        assert_eq!(flag, true);
    }

    #[test]
    fn test_is_prime_20() {
        let flag = is_prime(20);
        assert_eq!(flag, false);
    }

    #[test]
    fn test_enum_dividors_12() {
        let ans = enum_divisors(12);
        assert_eq!(ans, vec![1, 2, 3, 4, 6, 12]);
    }

    #[test]
    fn test_prime_factorize_2020() {
        let vec = prime_factorize(2020);
        assert_eq!(vec, vec![(2, 2), (5, 1), (101, 1)]);
    }

    #[test]
    fn test_eratosthenes_1() {
        let vec = sieve_of_eratosthenes(1);
        assert_eq!(vec, vec![]);
    }

    #[test]
    fn test_eratosthenes_2() {
        let vec = sieve_of_eratosthenes(2);
        assert_eq!(vec, vec![2]);
    }

    #[test]
    fn test_eratosthenes_20() {
        let vec = sieve_of_eratosthenes(20);
        assert_eq!(vec, vec![2, 3, 5, 7, 11, 13, 17, 19]);
    }
}