# Top N selector

This crate provides a _topset_ which selects a given number of greatest items.
The criterium used to sort the items could be specified as a closure.
It is based internally on a binary heap with a fixed size.

## Top N iterator

This is convenient way to build an iterator around
the N greatest items of a collection.

_Note:_ the returned items are unsorted.

```rust
use topset::TopSet;


fn main()
{
    let items = vec![4, 5, 8, 3, 2, 1, 4, 7, 9, 8];
    
    // getting the four greatest integers (repeating allowed)
    TopIter::new(4)
        .with_init(items)
        .into_iter()
        .for_each(|x| eprintln!("in the top 4: {}", x));
    
    // getting the four smallest integers
    // (we need to reverse the comparison function)
    TopIter::new(4)
        .with_init(items)
        .with_compare(|a,b| b.cmp(a))
        .into_iter()
        .for_each(|x| eprintln!("in the last 4: {}", x));
}
```
will produce (possibly in an different order):
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

## Top N set

Instead of manipulation an iterator, this provides a set containing the N greatest items.
