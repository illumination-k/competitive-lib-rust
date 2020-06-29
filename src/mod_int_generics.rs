use num_traits::{ PrimInt, NumAssign };

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};


pub struct ModInt<T: PrimInt + NumAssign> {
    pub int: T,
    MOD: T,
}

impl<T: PrimInt + NumAssign> Add<ModInt<T>> for ModInt<T> {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        let mut val = self.int + other.int;
        if (val >= self.MOD ) { val -= self.MOD };
        Self {
            int: val,
            MOD: self.MOD,
        }
    }
}

impl<T: PrimInt + NumAssign> Add<T> for ModInt<T> {
    type Output = Self;
    fn add(self, other: T) -> Self::Output {
        let mut val = self.int + other;
        if (val >= self.MOD ) { val -= self.MOD };
        Self {
            int: val,
            MOD: self.MOD,
        }
    }
}

impl<T: PrimInt + NumAssign> Sub<ModInt<T>> for ModInt<T> {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        let val = if (other.int > self.int) {
            self.MOD + self.int - other.int
        } else {
            self.int - other.int
        };
        Self {
            int: val,
            MOD: self.MOD
        }
    }
}




impl<T: PrimInt + NumAssign> ModInt<T> {
    fn new(int: T, MOD: T) -> Self {
        Self {
            int: int % MOD,
            MOD: MOD,
        }
    }
}


#[cfg(test)]
mod test {
    use super::ModInt;
    const MOD: usize = 1000000007;

    #[test]
    fn test_add() {
        let a = ModInt::new(1000000000, MOD);
        let b = ModInt::new(7, MOD);
        assert_eq!((a + 7).int, 0)
    }
}