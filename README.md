# xor-distance-exercise
Xor distances exercise including xor and bitwise operations for Rust.

### Intro
In order to get more familiar with **xor operations** and **xor distances**, you can try the following exercise. It is based on the challenge I’ve received as part of an interview.

This crate consists of:
- generic solution for the task (described below)
- starting point/code to take on the challenge (placed in [data](https://github.com/dalibor-matura/xor-distance-exercise/tree/master/data) folder)

### Task
Xor space has an odd shape, instead of location coordinates, places are specified by an unsigned `64-bit` integer. The distance between two points `x` and `y` is not what you’d expect, though: it’s `x ^ y`, the bitwise xor of the two locations.

There are entrepreneurs in xor space too and one of them came up with an idea for a local fresh food delivery system:
```rust
pub struct FoodDeliverySystem {
    addresses: Vec<u64>,
}
```
The idea led to a creation of an application. You enter your `position` and a `count` of the closest farms you want to get your food delivered from. It calculate addresses of the nearest farms, ordered from the closest to the `n-th` closest:
```rust
    pub fn closest_farms(&self, position: u64, count: usize) -> Vec<u64> {
        let mut sorted_farms = self.addresses.clone();
        sorted_farms.sort_by_key(|address| *address ^ position);
        sorted_farms.truncate(count);
        sorted_farms
    }
```
The information is handed over to delivery driver and farmers, so that the driver can pick up the food from the furthest farm to the nearest one to maximize freshness of the food when delivered.

Farmers are curious though and they’ve asked you to write an efficient (better than `O(n2)`) function that, given closest farms addresses, returns a possible `position` of the customer (there might be more than one such a position, but it just needs to return one of them). If there is no such a `position`, it should return `None`:
```rust
    pub fn reverse_closest_farms(&self, closest_farms: &[u64]) -> Option<u64> {
        // TODO: This is the part of an exercise you should implement.
        None
    }
```

### Instructions

1. Create a new empty project: `cargo new xor-distance-exercise-impl --lib`
2. Replace its `src/lib.rs` with the [lib.rs](https://github.com/dalibor-matura/xor-distance-exercise/blob/master/data/lib.rs) file (providing structure and tests) from [data](https://github.com/dalibor-matura/xor-distance-exercise/tree/master/data) folder.
3. Replace its `Cargo.toml` with the [Cargo.toml](https://github.com/dalibor-matura/xor-distance-exercise/blob/master/data/Cargo.toml) from [data](https://github.com/dalibor-matura/xor-distance-exercise/tree/master/data) folder.
4. Implement body of the `FoodDeliverySystem::reverse_closest_farms` method to make all tests pass.
