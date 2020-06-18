use cargo_snippet::snippet;

#[snippet("is_prime")]
pub fn is_prime(n: usize) -> bool {
    let mut flag: bool = true;

    if n == 1 { flag = false }
    let mut i: usize = 2;
    while i * i <= n {
        if n % i == 0 {
            flag = false;
            break;
        }
        i += 1;
    }
    flag
}

#[snippet("enum_divisors")]
pub fn enum_divisors(n: usize) -> Vec<usize> {
    let mut res: Vec<usize> = Vec::new();

    let mut i: usize = 1;

    while i * i <= n {
        if n % i == 0 {
            res.push(i);
            if n / i != i { res.push(n / i) }
        }
        i += 1;
    }
    res.sort();
    res
}


#[snippet("prime_factorize")]
pub fn prime_factorize(mut n: usize) -> Vec<(usize, usize)> {
    let mut res: Vec<(usize, usize)> = Vec::new();

    let mut i: usize = 2;

    while i * i <= n {
        if n % i == 0 {
            let mut ex: usize = 0;

            while n % i == 0 {
                ex += 1;
                n = n / i;
            }
            res.push((i, ex));
        }
        i += 1;
    }

    if n != 1 {res.push((n, 1))}

    res
}


#[snippet("sieve_of_eratosthenes")]
pub fn sieve_of_eratosthenes(n: usize) -> Vec<usize> {
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