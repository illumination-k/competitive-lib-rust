#[derive(Debug, Clone)]
pub struct Combination {
    fact_inv: Vec<usize>,
    inv: Vec<usize>,
    com: Option<Vec<usize>>,
    modulo: usize,
}

impl Combination {
    pub fn new(upper: usize, modulo: usize) -> Self {
        let mut fact_inv = vec![0; upper+1];
        let mut inv = vec![0; upper+1];
        fact_inv[0] = 1;
        fact_inv[1] = 1;
        inv[1] = 1;
        
        for i in 2..=upper {
            inv[i] = modulo - inv[modulo % i] * (modulo / i) % modulo;
            fact_inv[i] = fact_inv[i - 1] * inv[i] % modulo;
        }
        Self {
            fact_inv: fact_inv,
            inv: inv,
            com: None,
            modulo: modulo,
        }
    }

    pub fn fix_n(&mut self, n: usize) {
        let mut com = vec![0; n+1];
        com[0] = 1;
        for i in 1..=n {
            com[i] = com[i - 1] * ((n - i + 1) * self.inv[i] % self.modulo) % self.modulo;
        }
        self.com = Some(com)
    }

    fn _calc_nck(&self, n: usize, k: usize) -> usize {
        if n < k { return 0 }
        let mut ans: usize = 1;
        let mut i = n;

        while n - k < i {
            ans *= i;
            ans %= self.modulo;
            i -= 1;
        }
        ans * self.fact_inv[k] % self.modulo
    }

    pub fn nck(&self, n: usize, k: usize) -> usize {
        match &self.com {
            None => self._calc_nck(n, k),
            Some(x) => x[k],
        }
    }

    pub fn nhk(&self, n: usize, k: usize) -> usize {
        assert!(self.fact_inv.len() >= n+k-1);
        self.nck(n+k-1, k)
    }
}

/// simple calculation of combinations without modulo
/// ```
/// use competitive::calc_comb::combination;
/// let res = combination(16);
/// // get 16C11
/// assert_eq!(res[16][11], 4368);
/// // get 5C2
/// assert_eq!(res[5][2], 10);
/// ```
pub fn combination(n: usize) -> Vec<Vec<usize>> {
    let mut v = vec![vec![0; n+1]; n+1];

    for i in 0..n+1 {
        v[i][0] = 1;
        v[i][i] = 1;
    }

    for j in 1..n+1 {
        for k in 1..j {
            v[j][k] = v[j-1][k-1] + v[j-1][k]
        }
    }

    v
}


#[cfg(test)]
mod test {
    use super::Combination;

    const MOD: usize = 1e9 as usize + 7;

    #[test]
    fn n6ck() {
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
