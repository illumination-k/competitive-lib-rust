/*!
competitive library for atcoder.

[API Documentation](https://illumination-k.github.io/competitive-lib-rust/competitive)

## expand.py

```bash
exapnd.py --bin a
```

This python script can expand modules in a.rs.  

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
*/

#[cfg(test)]
#[macro_use]
extern crate approx;

extern crate num_traits;

pub mod dp;
pub mod mod_int;
pub mod prime;
pub mod graph2d;
pub mod graph;
pub mod combinations;
pub mod math;
pub mod scanner;
pub mod data_structures;
pub mod geometry;