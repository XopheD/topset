use std::fmt::{Debug, Formatter};
use std::mem;

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
/// will select the top elements and [`PartialOrd::lt`] will select the lowest.
///
/// Of course, any closure could be used but it should satisfy the transitivity.
/// In other words, if `a` beats `b` and `b` beats `c` then `a` should beat `c` too.
///
#[derive(Clone)]
pub struct TopSet<X,C>
{
    heap: Vec<X>, // a heap with the greatest at the end
    count: usize,
    beat: C
}

impl<X,C> TopSet<X,C>
    where C: Fn(&X,&X) -> bool
{
    /// Creates a new top set with a selecting closure.
    pub fn new(n: usize, beat: C) -> Self
    {
        assert!(n > 0);
        Self {
            heap: Vec::with_capacity(n),
            count: n,
            beat
        }
    }

    /// Check if the top set is empty
    #[inline]
    pub fn is_empty(&self) -> bool { self.heap.is_empty() }

    /// Get the number of stored items.
    ///
    /// It never exceeds the predefined capacity.
    #[inline]
    pub fn len(&self) -> usize { self.heap.len() }

    /// Get the capacity of this top set
    ///
    /// This capacity could only change by calling [`resize`].
    #[inline]
    pub fn capacity(&self) -> usize { self.count }

    /// Creates a new top set with a selecting closure and an initial set of items.
    ///
    /// If the initial set contains more than `n` elements, only the `n` greatest ones
    /// (according to `beat` selector) are stored.
    pub fn with_init<I: IntoIterator<Item=X>>(n: usize, init: I, beat: C) -> Self
    {
        assert!(n > 0);
        let mut top = Self::new(n, beat);
        top.extend(init);
        top
    }


    /// Resize the top set
    ///
    /// If the size decreases, then the lowest items are removed.
    /// If the size increases, nothing else happens but there is still more room
    /// for next insertions.
    pub fn resize(&mut self, n: usize)
    {
        if self.count < n {
            self.heap.reserve(n - self.count);
        } else {
            while self.heap.len() > n {
                self.pop();
            }
        }
        self.count = n;
    }

    /// Insert a new item.
    ///
    /// If the top set is not filled, the item is simply added and `None` is returned.
    ///
    /// If there is no more room, then one item should be rejected:
    /// * if the new item is better than some already stored ones, it is added and
    /// and the removed item is returned
    /// * if the new item is worse than all the stored ones, it is returned
    pub fn insert(&mut self, mut x: X) -> Option<X>
    {
        if self.heap.len() < self.count {
            // some room left
            self.heap.push(x);
            self.percolate_up(self.heap.len()-1);
            None

        } else {
            if (self.beat)(&x, &self.heap[0]) {
                // put the greatest the deepest: the new one should be kept
                mem::swap(&mut x, &mut self.heap[0]);
                self.percolate_down(0);
            }
            Some(x)
        }
    }

    /// Read access to the lowest item of the top set
    ///
    /// Notice that it actually returned the _lowest_ one and
    /// so all the others are better (or equal) this one.
    #[inline]
    pub fn peek(&self) -> Option<&X>
    {
        self.heap.first()
    }

    /// Pop the lowest item of the top set
    ///
    /// Remove and return the _lowest_ item of the top set.
    /// After this call, there is one more room for a item.
    ///
    /// This method is the only way to get the top elements
    /// in a sorted way (from the lowest to the best).
    pub fn pop(&mut self) -> Option<X>
    {
        if self.heap.len() <= 2 {
            self.heap.pop()
        } else {
            let pop = self.heap.swap_remove(0);
            self.percolate_down(0);
            Some(pop)
        }
    }

    /// Iterate over all the top selected items.
    ///
    /// The iterator is **not** sorted. A sorted iteration
    /// could be obtained by iterative call to [`Self::pop`].
    ///
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item=&X>
    {
        self.heap.iter()
    }


    // internal stuff
    fn percolate_up(&mut self, mut i: usize)
    {
        while i > 0 { // so has a parent (not root)
            let parent = (i-1)/2;
            // put the greatest the deepest
            if (self.beat)(&self.heap[parent], &self.heap[i]) {
                self.heap.swap(parent, i);
                i = parent;
            } else {
                break;
            }
        }
    }

    // internal stuff
    fn percolate_down(&mut self, mut i: usize)
    {
        loop {
            let mut child = 2*i+1;
            if child < self.heap.len()-1 {
                // to put the greatest the deepest -> select the greatest child
                if (self.beat)(&self.heap[child], &self.heap[child+1]) {
                    child += 1;
                }
                // put the greatest the deepest
                if (self.beat)(&self.heap[i], &self.heap[child]) {
                    self.heap.swap(i, child);
                    i = child;
                } else {
                    break;
                }
            } else {
                if (child == self.heap.len() - 1) && (self.beat)(&self.heap[i], &self.heap[child]) {
                    // only one child
                    self.heap.swap(i, child);
                }
                // end of heap
                break;
            }
        }
    }
}


impl<X,C> IntoIterator for TopSet<X,C>
{
    type Item = X;
    type IntoIter = <Vec<X> as IntoIterator>::IntoIter;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.heap.into_iter()
    }
}

impl<X,C> Extend<X> for TopSet<X,C>
    where C: Fn(&X,&X) -> bool
{
    fn extend<T: IntoIterator<Item=X>>(&mut self, iter: T) {
        iter.into_iter()
            .for_each(|x| { self.insert(x); } )
    }
}

impl<X,C> Debug for TopSet<X,C>
    where X:Debug
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.heap.fmt(f)
    }
}



#[cfg(test)]
mod tests {
    use crate::TopSet;

    #[test]
    fn cost()
    {
        let mut top = TopSet::<f32,_>::new(5, f32::lt);
        top.extend(vec![81.5, 4.5,4.,1.,45.,22.,11.]);
        dbg!(&top);
        top.extend(vec![81.5, 4.5,4.,1.,45.,22.,11.]);
        dbg!(top);
    }

    #[test]
    fn ord()
    {
        let mut top = TopSet::<u32,_>::new(5, u32::gt);
        top.extend(vec![81,5, 4,5,4,1,45,22,1,5,97,5,877,12,0]);
        dbg!(&top);
    }
}