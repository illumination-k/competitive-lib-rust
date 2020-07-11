use cargo_snippet::snippet;

#[snippet("struct_comb")]
#[derive(Debug, Clone)]
struct Combination {
    fact_inv: Vec<usize>,
    inv: Vec<usize>,
    com: Option<Vec<usize>>,
    MOD: usize,
}

#[snippet("calc_comb")]
#[snippet(include="struct_comb")]
impl Combination {
    fn new(n: usize, MOD: usize) -> Combination {
        let mut fact_inv = vec![0; n+1];
        let mut inv = vec![0; n+1];
        fact_inv[0] = 1;
        fact_inv[1] = 1;
        inv[1] = 1;
        
        for i in 2..=n {
            inv[i] = MOD - inv[MOD % i] * (MOD / i) % MOD;
            fact_inv[i] = fact_inv[i - 1] * inv[i] % MOD;
        }
        Combination {
            fact_inv: fact_inv,
            inv: inv,
            com: None,
            MOD: MOD,
        }
    }

    fn fix_n(&mut self, n: usize) {
        let mut com = vec![0; n+1];
        com[0] = 1;
        for i in 1..=n {
            com[i] = com[i - 1] * ((n - i + 1) * self.inv[i] % self.MOD) % self.MOD;
        }
        self.com = Some(com)
    }

    fn _calc_nck(&self, n: usize, k: usize) -> usize {
        if n < k { return 0 }
        let mut ans: usize = 1;
        let mut i = n;

        while n - k < i {
            ans *= i;
            ans %= self.MOD;
            i -= 1;
        }
        ans * self.fact_inv[k] % self.MOD
    }

    fn nck(&self, n: usize, k: usize) -> usize {
        match self.com.clone() {
            None => self._calc_nck(n, k),
            Some(x) => x[k],
        }
    }

    fn nhk(&self, n: usize, k: usize) -> usize {
        if self.fact_inv.len() < n+k-1 {
            panic!("length must be n+k-1 or larger!")
        }
        self.nck(n+k-1, k)
    }
}


#[cfg(test)]
mod test {
    use super::Combination;

    const MOD: usize = 1e9 as usize + 7;

    #[test]
    fn n6Ck() {
        let comb = Combination::new(6, MOD);
        assert_eq!(comb.nck(0, 0), 1);
        assert_eq!(comb.nck(6, 2), 15);
        assert_eq!(comb.nck(6, 0), 1);
        assert_eq!(comb.nck(6, 3), 20);
        assert_eq!(comb.nck(6, 6), 1);
        assert_eq!(comb.nck(4, 2), 6);
        assert_eq!(comb.nck(5, 5), 1);
        assert_eq!(comb.nck(5, 3), 10);
    }

    #[test]
    fn fixn10ck() {
        let mut comb = Combination::new(10, MOD);
        comb.fix_n(10);
        assert_eq!(comb.nck(10, 5), 252);
        assert_eq!(comb.nck(10, 10), 1);
        assert_eq!(comb.nck(10, 0), 1);
    }

    #[test]
    fn nhk() {
        let comb = Combination::new(100, MOD);
        assert_eq!(comb.nhk(3, 4), 15);
    }
}