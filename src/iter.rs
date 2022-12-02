use std::iter::{FusedIterator};
use crate::TopSet;

pub struct IntoIterSorted<X:PartialEq,C>(TopSet<X,C>)
    where C: FnMut(&X,&X) -> bool;

impl<X:PartialEq,C> From<TopSet<X,C>> for IntoIterSorted<X,C>
    where C: FnMut(&X,&X) -> bool
{
    #[inline] fn from(topset: TopSet<X, C>) -> Self { Self(topset) }
}

impl<X:PartialEq,C> IntoIterSorted<X,C>
    where C: FnMut(&X,&X) -> bool
{
    #[inline] pub fn peek(&mut self) -> Option<&X> { self.0.peek() }
}

impl<X:PartialEq,C> Iterator for IntoIterSorted<X,C>
    where C: FnMut(&X,&X) -> bool
{
    type Item = X;
    #[inline] fn next(&mut self) -> Option<Self::Item> { self.0.pop() }
    #[inline] fn count(self) -> usize { self.0.len() }
    #[inline] fn size_hint(&self) -> (usize, Option<usize>) { (self.0.len(), Some(self.0.len())) }
    #[inline] fn last(self) -> Option<X> {
        self.0.heap.into_iter().reduce(|a,b| if (self.0.beat.borrow_mut())(&a,&b) {a} else {b})
    }
}

impl<X:PartialEq,C:FnMut(&X,&X)->bool> FusedIterator for IntoIterSorted<X,C> { }

// impl<X,C:FnMut(&X,&X)->bool> TrustedLen for IntoIterSorted<X,C> { }

impl<X:PartialEq,C> ExactSizeIterator for IntoIterSorted<X,C>
    where C: FnMut(&X,&X) -> bool
{
    #[inline] fn len(&self) -> usize { self.0.len() }

    // #[inline] fn is_empty(&self) -> bool { self.0.is_empty() }
}

pub trait TopSetReducing
{
    type Item:PartialEq;

    fn topset<C>(self, n: usize, beat: C) -> TopSet<Self::Item, C>
        where C: FnMut(&Self::Item, &Self::Item) -> bool;
}

impl<I:Iterator> TopSetReducing for I
    where I::Item : PartialEq
{
    type Item = I::Item;

    #[inline]
    fn topset<C>(self, n: usize, beat: C) -> TopSet<Self::Item, C>
        where C: FnMut(&Self::Item, &Self::Item) -> bool
    {
        self.fold(TopSet::new(n,beat), |mut top, e| { top.insert(e); top })
    }
}

