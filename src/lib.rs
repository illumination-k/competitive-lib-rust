/*!
competitive library for atcoder.

[API Documentation](https://illumination-k.github.io/competitive-lib-rust/competitive)

## expand.py

```bash
exapnd.py --bin a
```
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