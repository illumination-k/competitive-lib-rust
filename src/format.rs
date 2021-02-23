use std::vec::Vec;

/// Trait of format for atcoder
///    
/// bool -> Yes or No  
/// vec![a, b ,c] -> "a b c"  
/// vec![vec![0, 1], vec![1, 0]] -> "0 1\n1 0"  
pub trait AtCoderFormat {
    fn format(&self) -> String;
}

macro_rules! impl_format {
    ($t: ty) => {
        impl AtCoderFormat for $t {
            fn format(&self) -> String {
                self.to_string()
            }
        }

        impl AtCoderFormat for Vec<$t> {
            fn format(&self) -> String {
                self.iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join("\n")
            }
        }

        impl AtCoderFormat for Vec<Vec<$t>>
        {
            fn format(&self) -> String {
                self.iter()
                    .map(|x| {
                        x.iter()
                            .map(|x| x.to_string())
                            .collect::<Vec<String>>()
                            .join(" ")
                    })
                    .collect::<Vec<String>>()
                    .join("\n")
            }
        }
    };
}

impl_format!(usize);
impl_format!(u128);
impl_format!(u64);
impl_format!(u32);
impl_format!(u16);
impl_format!(u8);
impl_format!(isize);
impl_format!(i128);
impl_format!(i64);
impl_format!(i32);
impl_format!(i16);
impl_format!(i8);
impl_format!(&str);
impl_format!(String);
impl_format!(char);

impl AtCoderFormat for bool {
    fn format(&self) -> String {
        if self == &true {
            "Yes".to_string()
        } else {
            "No".to_string()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_unsigned_int() {
        let i: usize = 1;
        assert_eq!(i.format(), 1.to_string())
    }

    #[test]
    fn test_vec() {
        let vi = vec![1, 2, 3];
        assert_eq!(vi.format(), "1\n2\n3".to_string());
        let vc = vec!['a', 'b', 'c'];
        assert_eq!(vc.format(), "a\nb\nc".to_string());
        let vs = vec!["ab", "cd", "ef"];
        assert_eq!(vs.format(), "ab\ncd\nef".to_string());
        let vst = vec!["ab".to_string(), "cd".to_string(), "ef".to_string()];
        assert_eq!(vst.format(), "ab\ncd\nef".to_string());
    }

    #[test]
    fn test_vec2d() {
        let v = vec![vec![1, 2], vec![2, 1]];
        assert_eq!(v.format(), "1 2\n2 1")
    }

    #[test]
    fn test_bool() {
        assert_eq!(true.format(), "Yes".to_string());
        assert_eq!(false.format(), "No".to_string());
    }

    #[test]
    fn test_solve() {
        fn solve_unsigned_int() -> Option<impl AtCoderFormat> {
            Some(1)
        }

        assert_eq!(solve_unsigned_int().unwrap().format(), "1".to_string());

        fn solve_vec() -> Option<impl AtCoderFormat> {
            Some(vec![1, 2, 3])
        }
    }
}