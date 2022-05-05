use std::fmt::{Debug, Formatter};
use std::mem;

/// A top N set of items.
///
/// This set contains no more than N items.
/// When this limit is reached, the smallest (according to
/// the specified comparison) is thrown.
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
    /// Creates a new top set with a
    pub fn new(n: usize, beat: C) -> Self
    {
        assert!(n > 0);
        Self {
            heap: Vec::with_capacity(n),
            count: n,
            beat
        }
    }

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

    pub fn with_init<I: IntoIterator<Item=X>>(n: usize, init: I, beat: C) -> Self
    {
        assert!(n > 0);
        let mut top = Self::new(n, beat);
        top.extend(init);
        top
    }


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

    pub fn push(&mut self, mut x: X) -> Option<X>
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

    pub fn peek(&self) -> Option<&X>
    {
        self.heap.first()
    }

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

    pub fn iter(&self) -> impl Iterator<Item=&X>
    {
        self.heap.iter()
    }
}


impl<X,C> IntoIterator for TopSet<X,C>
{
    type Item = X;
    type IntoIter = <Vec<X> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.heap.into_iter()
    }
}

impl<X,C> Extend<X> for TopSet<X,C>
    where C: Fn(&X,&X) -> bool
{
    fn extend<T: IntoIterator<Item=X>>(&mut self, iter: T) {
        iter.into_iter()
            .for_each(|x| { self.push(x); } )
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