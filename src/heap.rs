use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};
use std::mem;
use crate::TopSet;

impl<X,C> TopSet<X,C>
    where C: FnMut(&X,&X) -> bool
{
    /// Creates a new top set with a selecting closure.
    ///
    /// The size corresponds to the maximum number of items
    /// allowed in the top set.
    ///
    /// The function `C` is the challenge to decide if an
    /// element beats another one and so takes its place.
    /// It should corresponds to a total ordering.
    ///
    /// If the first one beats the second one then `true` should
    /// be returned and if `false' corresponds to the case when
    /// the second item beats the first one.
    /// This function should always returns the same result
    /// when dealing with the same items or results are unpredictable.
    ///
    /// ## Example
    /// Collecting the 5 greatest integers is performed by using a
    /// topset with `n = 5` and `beat = i32::gt`.
    pub fn new(n: usize, beat: C) -> Self
    {
        Self {
            heap: Vec::with_capacity(n),
            count: n,
            beat: beat.into()
        }
    }

    /// Creates a new top set with a selecting closure and an initial set of items.
    ///
    /// If the initial set contains more than `n` elements, only the `n` greatest ones
    /// (according to `beat` challenging function) are stored.
    pub fn with_init<I: IntoIterator<Item=X>>(n: usize, init: I, beat: C) -> Self
    {
        let mut top = Self::new(n, beat);
        top.extend(init);
        top
    }

    /// Check if the top set is empty
    #[inline]
    pub fn is_empty(&self) -> bool { self.heap.is_empty() }

    /// Get the number of stored items.
    ///
    /// It never exceeds the predefined capacity
    /// (the capacity does not grow by itself, only by calling [`Self::resize`]).
    #[inline]
    pub fn len(&self) -> usize { self.heap.len() }

    /// Get the capacity of this top set
    ///
    /// The capacity limits the number of elements to keep.
    /// This capacity could only change by calling [`resize`].
    #[inline]
    pub fn capacity(&self) -> usize { self.count }

    /// Read access to the lowest item of the top set
    ///
    /// Notice that it actually returned the _lowest_ one and
    /// so all the others are better (or equal) this one.
    #[inline]
    pub fn peek(&self) -> Option<&X>
    {
        self.heap.first()
    }

    /// Checks if an item will be inserted or not
    ///
    /// If it `true` is returned, it means that a call to [`Self::insert`]
    /// will actually insert the candidate. If `false`, then the insertion
    /// will be a non-op.
    ///
    /// Note that in any case the insertion is not done. See [`Self::insert`] to
    /// perform the test and the insertion in one time.
    #[inline]
    pub fn is_candidate(&self, x: &X) -> bool {
        self.heap.len() < self.count || self.beat(x, self.peek().unwrap())
    }

    /// Iterate over all the top selected items.
    ///
    /// The iterator is **not** sorted. A sorted iteration
    /// could be obtained by iterative call to [`Self::pop`]
    /// or by using [`Self::into_iter_sorted`].
    ///
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item=&X>
    {
        self.heap.iter()
    }

    /// Gets all the top set elements in a vector.
    ///
    /// This vector is **not** sorted.
    /// See [`Self::into_sorted_vec`] if a sorted result is expected.
    #[inline]
    pub fn into_vec(self) -> Vec<X> { self.heap }

    /// Insert a new item.
    ///
    /// If the top set is not filled (i.e. its length is less than its capacity),
    /// the item is simply added and `None` is returned.
    ///
    /// If there is no more room, then one item should be rejected:
    /// * if the new item is better than some already stored ones, it is added
    /// and the removed item is returned
    /// * if the new item is worse than all the stored ones, it is returned
    ///
    pub fn insert(&mut self, mut x: X) -> Option<X>
    {
        if self.heap.len() < self.count {
            // some room left, so nothing to remove
            self.heap.push(x);
            self.percolate_up(self.heap.len()-1);
            None
        } else {
            // SAFETY: if the heap is empty when self.count != 0, then we fall
            // in the previous if condition (so, here, get_unchecked is safe)
            if self.count != 0 && self.beat(&x, unsafe { self.heap.get_unchecked(0) }) {
                // put the greatest the deepest: the new one should be kept
                mem::swap(&mut x, &mut self.heap[0]);
                self.percolate_down(0);
            }
            Some(x)
        }
    }

    /// Converts this topset into a sorted iterator
    ///
    /// Notice that the _lowest_ item of the top set is the
    /// first one. The _greatest_ item is the last one.
    #[inline]
    pub fn into_iter_sorted(self) -> crate::iter::IntoIterSorted<X,C> {
        self.into()
    }

    /// Returns the topset in a sorted vector.
    ///
    /// The first element of the vector is the _lowest_ item of the top set
    /// and the last one is the _greatest_ one.
    pub fn into_sorted_vec(mut self) -> Vec<X>
        where X:PartialEq
    {
        self.heap.sort_unstable_by(|a,b| {
            if *a == *b {
                Ordering::Equal
            } else if (self.beat.borrow_mut())(a,b) {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        });
        self.heap
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

    /// Pop the lowest item of the top set
    ///
    /// Remove and return the _lowest_ item of the top set.
    /// After this call, there is one more room for a item.
    ///
    /// This method is the only way to get the top elements
    /// in a sorted way (from the lowest to the best).
    pub fn pop(&mut self) -> Option<X>
    {
        match self.heap.len() {
            0 => None,
            1|2 => Some(self.heap.swap_remove(0)),
            _ => {
                let pop = self.heap.swap_remove(0);
                self.percolate_down(0);
                Some(pop)
            }
        }
    }

    /// Removes all the elements in the top set
    #[inline] pub fn clear(&mut self) { self.heap.clear() }

    /// Checks if an element beats the other
    #[inline] pub fn beat(&self, a:&X, b:&X) -> bool { self.beat.borrow_mut()(a,b) }

    // internal stuff
    // move i up (to the best)
    fn percolate_up(&mut self, mut i: usize)
    {
        while i > 0 { // so has a parent (not root)
            let parent = (i-1)/2;
            // put the greatest the deepest
            if self.beat(&self.heap[parent], &self.heap[i]) {
                self.heap.swap(parent, i);
                i = parent;
            } else {
                break;
            }
        }
    }

    // internal stuff
    // move i as deep as possible
    fn percolate_down(&mut self, mut i: usize)
    {
        loop {
            let mut child = 2*i+1;
            if child < self.heap.len()-1 {
                // to put the greatest the deepest -> select the greatest child
                if self.beat(&self.heap[child], &self.heap[child+1]) {
                    child += 1;
                }
                // put the greatest the deepest
                if self.beat(&self.heap[i], &self.heap[child]) {
                    self.heap.swap(i, child);
                    i = child;
                } else {
                    break;
                }
            } else {
                if (child == self.heap.len() - 1) && self.beat(&self.heap[i], &self.heap[child]) {
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
    where C: FnMut(&X,&X) -> bool
{
    type Item = X;
    type IntoIter = <Vec<X> as IntoIterator>::IntoIter;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.heap.into_iter()
    }
}

impl<'a,X,C> IntoIterator for &'a TopSet<X,C>
    where C: FnMut(&X,&X) -> bool
{
    type Item = &'a X;
    type IntoIter = <&'a Vec<X> as IntoIterator>::IntoIter;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        (&self.heap).into_iter()
    }
}

impl<X,C> Extend<X> for TopSet<X,C>
    where C: FnMut(&X,&X) -> bool
{
    #[inline]
    fn extend<T: IntoIterator<Item=X>>(&mut self, iter: T) {
        iter.into_iter().for_each(|x| { self.insert(x); } )
    }
}


impl<X,C> Debug for TopSet<X,C>
    where X:Debug, C: FnMut(&X,&X) -> bool
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.heap.fmt(f)
    }
}



#[cfg(test)]
mod tests {
    use crate::iter::TopSetReducing;
    use crate::TopSet;

    #[test]
    fn lowest_cost()
    {
        let mut top = TopSet::<f32,_>::new(5, f32::lt);
        top.extend(vec![81.5, 4.5, 4., 1., 45., 22., 11.]);
        top.extend(vec![81.5, 4.5, 4., 1., 45., 22., 11.]);

        assert_eq![ top.pop(), Some(4.5) ];
        assert_eq![ top.pop(), Some(4.) ];
        assert_eq![ top.pop(), Some(4.) ];
        assert_eq![ top.pop(), Some(1.) ];
        assert_eq![ top.pop(), Some(1.) ];
        assert_eq![ top.pop(), None ];
    }

    #[test]
    fn greatest_score()
    {
        assert_eq![
            vec![81,5, 4,5,4,1,45,22,1,5,97,5,877,12,0]
            .into_iter()
            .topset(5, u32::gt)
            .into_iter()
            .last(),
            Some(877)];
    }

    /*    fn top()
    {
        let items = vec![4, 5, 9, 2, 3, 8, 4, 7, 8, 1];

        // getting the four greatest integers (repeating allowed)
        items.clone().into_iter()
            .topset(4, i32::gt)
            .into_iter_sorted()
            .for_each(|x| eprintln!("in the top 4: {}", x));

        let mut i =  items.clone().into_iter()
            .topset(4, i32::gt);
        eprintln!("in the top 4: {:?}", i.pop());
        eprintln!("in the top 4: {:?}", i.pop());
        eprintln!("in the top 4: {:?}", i.pop());
        eprintln!("in the top 4: {:?}", i.pop());
        eprintln!("in the top 4: {:?}", i.pop());
        eprintln!("in the top 4: {:?}", i.pop());
        eprintln!("in the top 4: {:?}", i.pop());

        // getting the four smallest integers
        // (we just need to reverse the comparison function)
        items.into_iter()
            .topset(4, i32::lt)
            .into_iter_sorted()
            .for_each(|x| eprintln!("in the last 4: {}", x));
    }*/
}