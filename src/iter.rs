use std::iter::{FusedIterator};
use crate::TopSet;

pub struct IntoIterSorted<X,C>(TopSet<X,C>)
    where C: Fn(&X,&X) -> bool;

impl<X,C> From<TopSet<X,C>> for IntoIterSorted<X,C>
    where C: Fn(&X,&X) -> bool
{
    #[inline] fn from(topset: TopSet<X, C>) -> Self { Self(topset) }
}

impl<X,C> IntoIterSorted<X,C>
    where C: Fn(&X,&X) -> bool
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

// impl<X,C:Fn(&X,&X)->bool> TrustedLen for IntoIterSorted<X,C> { }

impl<X,C> ExactSizeIterator for IntoIterSorted<X,C>
    where C: Fn(&X,&X) -> bool
{
    #[inline] fn len(&self) -> usize { self.0.len() }

    // #[inline] fn is_empty(&self) -> bool { self.0.is_empty() }
}

pub trait TopSetReducing
{
    type Item;

    /// Build the top set according to the specified challenge.
    fn topset<C>(self, n: usize, beat: C) -> TopSet<Self::Item, C>
        where C: Fn(&Self::Item, &Self::Item) -> bool;

    /// Build the top set of the greatest values.
    #[inline]
    fn topset_greatest(self, n: usize) -> TopSet<Self::Item, fn(&Self::Item,&Self::Item)->bool>
        where Self::Item: PartialOrd, Self: Sized
    {
        self.topset(n, <Self::Item as PartialOrd>::gt)
    }

    /// Build the top set of the lowest values.
    #[inline]
    fn topset_lowest(self, n: usize) -> TopSet<Self::Item, fn(&Self::Item,&Self::Item)->bool>
        where Self::Item: PartialOrd, Self: Sized
    {
        self.topset(n, <Self::Item as PartialOrd>::lt)
    }
}

impl<I:IntoIterator> TopSetReducing for I
{
    type Item = I::Item;

    #[inline]
    fn topset<C>(self, n: usize, beat: C) -> TopSet<Self::Item, C>
        where C: Fn(&Self::Item, &Self::Item) -> bool
    {
        self.into_iter().fold(TopSet::new(n,beat), |mut top, e| { top.insert(e); top })
    }
}


#[cfg(test)]
mod tests {
    use crate::iter::TopSetReducing;

    #[test]
    fn lowest_cost()
    {
        let mut top = vec![
            81.5, 4.5, 4., 1., 45., 22., 11., 81.5, 4.5, 4., 1., 45., 22., 11.
        ].topset_lowest(5);

        assert_eq![top.pop(), Some(4.5)];
        assert_eq![top.pop(), Some(4.)];
        assert_eq![top.pop(), Some(4.)];
        assert_eq![top.pop(), Some(1.)];
        assert_eq![top.pop(), Some(1.)];
        assert_eq![top.pop(), None];
    }

    #[test]
    fn greatest_score()
    {
        assert_eq![
            vec![81, 5, 4, 5, 4, 1, 45, 22, 1, 5, 97, 5, 877, 12, 0]
                .into_iter()
                .topset_greatest(5)
                .into_iter()
                .last(),
            Some(877)];
    }
}