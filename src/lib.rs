//! # Top N set
//!
//! This crate provides a _topset_ which selects a given number of greatest items.
//! The criterium used to sort the items could be specified as a closure.
//! It is based internally on a binary heap with a fixed size.
//!
//! ```
//! use topset::*;
//!
//! fn main()
//! {
//!     let items = vec![4, 5, 8, 3, 2, 1, 4, 7, 9, 8];
//!
//!     // getting the four greatest integers (repeating allowed)
//!     TopSet::with_init(4, items.iter().copied(), u32::gt)
//!         .into_iter().for_each(|x| eprintln!("in the top 4: {}", x));
//!
//!     // getting the four smallest integers
//!     // (we just need to reverse the comparison function)
//!     TopSet::with_init(4,items.into_iter(), u32::lt)
//!         .into_iter().for_each(|x| eprintln!("in the last 4: {}", x));
//! }
//! ```
//! will produce (possibly in an different order):
//! ```text
//! in the top 4: 7
//! in the top 4: 8
//! in the top 4: 9
//! in the top 4: 8
//! in the last 4: 4
//! in the last 4: 3
//! in the last 4: 1
//! in the last 4: 2
//! ```

pub mod iter;
mod heap;


/// A top N set of items.
///
/// This set contains no more than N items.
/// When this limit is reached, the smallest (according to
/// the specified comparison) is thrown.
///
/// Comparing two elements is done by a duel, resolved by a provided closure:
/// if `true` is returned, the first item wins, if `false` the second.
///
/// By the way, using [`PartialOrd::gt`]
/// will select the top elements and [`PartialOrd::lt`]
/// will select the lowest.
///
/// Of course, any closure could be used but it should satisfy the transitivity.
/// In other words, if `a` beats `b` and `b` beats `c` then `a` should beat `c` too.
/// If it is not the case, the results are unpredictable.
///
#[derive(Clone)]
pub struct TopSet<X,C>
{
    heap: Vec<X>, // a heap with the greatest at the end
    count: usize,
    beat: C
}


