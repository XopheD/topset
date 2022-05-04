use std::cmp::Ordering;
use std::cmp::Ordering::{Greater, Less};
use std::fmt::{Debug, Formatter};
use std::mem;

#[derive(Clone)]
pub struct TopSet<X,C>
{
    heap: Vec<X>, // a heap with the greatest at the end
    cmp: C
}

impl<X,C> TopSet<X,C>
    where C: Fn(&X,&X) -> Ordering
{
    pub fn new(n: usize, cmp: C) -> Self
    {
        assert!(n > 0);
        Self {
            heap: Vec::with_capacity(n),
            cmp
        }
    }

    pub fn with_init<I: IntoIterator<Item=X>>(n: usize, init: I, cmp: C) -> Self
    {
        assert!(n > 0);
        let mut top = Self::new(n, cmp);
        top.extend(init);
        top
    }

    fn percolate_up(&mut self, mut i: usize)
    {
        while i > 0 { // so has a parent (not root)
            let parent = (i-1)/2;
            // put the greatest the deepest
            if (self.cmp)(&self.heap[parent], &self.heap[i]) == Greater {
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
                if (self.cmp)(&self.heap[child], &self.heap[child+1]) == Greater {
                    child += 1;
                }
                // put the greatest the deepest
                if (self.cmp)(&self.heap[i], &self.heap[child]) == Greater {
                    self.heap.swap(i, child);
                    i = child;
                } else {
                    break;
                }
            } else {
                if (child == self.heap.len() - 1) && (self.cmp)(&self.heap[i], &self.heap[child]) == Greater {
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
        if self.heap.len() < self.heap.capacity() {
            // some room left
            self.heap.push(x);
            self.percolate_up(self.heap.len()-1);
            None

        } else {
            if (self.cmp)(&x, &self.heap[0]) == Greater {
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
    where C: Fn(&X,&X) -> Ordering
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
        let mut top = TopSet::<f32,_>::new(5, |a, b| b.partial_cmp(a).unwrap());
        top.extend(vec![81.5, 4.5,4.,1.,45.,22.,11.]);
        dbg!(&top);
        top.extend(vec![81.5, 4.5,4.,1.,45.,22.,11.]);
        dbg!(top);
    }

    #[test]
    fn ord()
    {
        let mut top = TopSet::<u32,_>::new(5, u32::cmp);
        top.extend(vec![81,5, 4,5,4,1,45,22,1,5,97,5,877,12,0]);
        dbg!(&top);
    }
}