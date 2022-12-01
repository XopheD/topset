use std::iter::{FusedIterator};
use crate::TopSet;

pub struct IntoIterSorted<X,C>(pub(crate) TopSet<X,C>);

impl<X,C> IntoIterSorted<X,C>
{
    #[inline] pub fn peek(&mut self) -> Option<&X> { self.0.peek() }
}

impl<X,C> Iterator for IntoIterSorted<X,C>
    where C: Fn(&X,&X) -> bool
{
    type Item = X;
    #[inline] fn next(&mut self) -> Option<Self::Item> { self.0.pop() }
    #[inline] fn count(self) -> usize { self.0.len() }
    #[inline] fn size_hint(&self) -> (usize, Option<usize>) { (self.0.len(), Some(self.0.len())) }
    #[inline] fn last(self) -> Option<X> {
        self.0.heap.into_iter().reduce(|a,b| if (self.0.beat)(&a,&b) {a} else {b})
    }
}

impl<X,C:Fn(&X,&X)->bool> FusedIterator for IntoIterSorted<X,C> { }

impl<X,C> ExactSizeIterator for IntoIterSorted<X,C>
    where C: Fn(&X,&X) -> bool
{
    #[inline] fn len(&self) -> usize { self.0.len() }

    // #[inline] fn is_empty(&self) -> bool { self.0.is_empty() }
}


// Unstable but it’s true !!!
// impl<X,C:Fn(&X,&X)->bool> TrustedLen for IntoIterSorted<X,C> { }
