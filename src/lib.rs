/*!
competitive library for atcoder.

[API Documentation](https://illumination-k.github.io/competitive-lib-rust/competitive)

## expand.py

```bash
exapnd.py --bin a
```

This python script can expand modules in a.rs. (expected directory made by `cargo-atcoder`).

### Examples

```ignore
use competitive::math::*;
```

Convert the above code as follows

```ignore
use competitive_internal::math::*;

mod competitive_internal {
    pub mod math {
        /* math codes! */
    }
}
```
*/

#[cfg(test)]
#[macro_use]
extern crate approx;

extern crate num_traits;
extern crate rand;

pub mod binary_search;
pub mod combinations;
pub mod data_structures;
pub mod dp;
pub mod format;
pub mod geometry;
pub mod graph;
pub mod graph2d;
pub mod math;
pub mod mod_int;
pub mod prime;
pub mod scanner;
pub mod test_utility;
