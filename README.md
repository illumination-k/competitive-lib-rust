# competitive

competitive library for atcoder.

[API Documentation](https://illumination-k.github.io/competitive-lib-rust/competitive)

### expand.py

```bash
exapnd.py --bin a
```

This python script can expand modules in a.rs .

#### Examples

```rust
use competitive::math::*;
```

Convert the above code as follows

```rust
use competitive_internal::math::*;

mod competitive_internal {
    pub mod math {
        /* math codes! */
    }
}
```
