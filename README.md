# Top N Set
[![Crates.io](https://img.shields.io/crates/v/topset?style=flat)](https://crates.io/crates/topset)
[![Crates.io](https://img.shields.io/crates/d/topset?style=flat)](https://crates.io/crates/topset)
[![License](https://img.shields.io/badge/license-MIT-blue?style=flat)](https://crates.io/crates/topset)
[![Docs](https://img.shields.io/docsrs/topset)](https://docs.rs/topset)

This crate provides a _topset_ which selects a given number of greatest items.
The criterium used to sort the items could be specified as a closure.
It is based internally on a binary heap with a fixed size.

The struct `TopSet` could be used directly or through the trait `TopSetReducing`
which automatically extend the iterator trait.

_Note:_ the returned items are unsorted.

```rust
use topset::TopIter;


fn main()
{
    let items = vec![4, 5, 8, 3, 2, 1, 4, 7, 9, 8];
    
    // getting the four greatest integers (repeating allowed)
    items.iter().cloned()
            .topset(4, i32::gt)
            .into_iter()
            .for_each(|x| eprintln!("in the top 4: {}", x));

    // getting the four smallest integers
    // (we just need to reverse the comparison function)
    items.topset(4, i32::lt)
            .into_iter()
            .for_each(|x| eprintln!("in the last 4: {}", x));
}
```
will produce (possibly in a different order):
```text
in the top 4: 7
in the top 4: 8
in the top 4: 9
in the top 4: 8
in the last 4: 4
in the last 4: 3
in the last 4: 1
in the last 4: 2
```
