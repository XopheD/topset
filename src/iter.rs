
use std::marker::PhantomData;
use crate::TopSet;

/// An iterator builder over the N greatest items of a given set
#[derive(Clone)]
pub struct TopIter { count: usize }

#[derive(Clone)]
pub struct TopIterI<I:IntoIterator> { count: usize, init: I }

#[derive(Clone)]
pub struct TopIterC<X,C:Fn(&X, &X)->bool> { count: usize, beat: C, item: PhantomData<X> }

#[derive(Clone)]
pub struct TopIterIC<I:IntoIterator,C:Fn(&I::Item, &I::Item)->bool> { top: TopSet<I::Item,C> }


impl Default for TopIter
{
    fn default() -> Self { Self::new(1) }
}

impl TopIter
{
    /// A Top N iterator
    #[inline] pub fn new(n: usize) -> Self {
        assert!( n > 0 );
        Self { count: n }
    }

    /// Specify the initial set of items to scan
    #[inline] pub fn with_init<I:IntoIterator>(self, init:I) -> TopIterI<I>
    {
        TopIterI { count: self.count, init }
    }

    /// Specify the comparison to use
    #[inline] pub fn with_selector<X, C:Fn(&X, &X)->bool>(self, beat:C) -> TopIterC<X,C>
    {
        TopIterC { count: self.count, beat, item: PhantomData::default() }
    }
}


impl<I:IntoIterator> TopIterI<I>
{
    #[inline] pub fn with_selector<C:Fn(&I::Item, &I::Item)->bool>(self, beat:C) -> TopIterIC<I,C>
    {
        TopIterIC { top: TopSet::with_init(self.count, self.init, beat) }
    }
}


impl<X,C:Fn(&X, &X)->bool> TopIterC<X,C>
{
    #[inline] pub fn with_init<I:IntoIterator<Item=X>>(self, init:I) -> TopIterIC<I,C>
    {
        TopIterIC { top: TopSet::with_init(self.count, init, self.beat) }
    }
}


impl<I> IntoIterator for TopIterI<I>
    where
        I: IntoIterator,
        I::Item : PartialOrd
{
    type Item = I::Item;
    type IntoIter = <Vec<I::Item> as IntoIterator>::IntoIter;

    #[inline] fn into_iter(self) -> Self::IntoIter {
        self.with_selector(<I::Item as PartialOrd>::gt).into_iter()
    }
}

impl <I:IntoIterator,C:Fn(&I::Item, &I::Item)->bool> IntoIterator for TopIterIC<I,C>
{
    type Item = I::Item;
    type IntoIter = <Vec<I::Item> as IntoIterator>::IntoIter;

    #[inline] fn into_iter(self) -> Self::IntoIter {
        self.top.into_iter()
    }
}


#[cfg(test)]
mod tests {
    use crate::TopIter;

    #[test]
    fn cost()
    {
        dbg!(TopIter::new(3)
            .with_selector(|a:&f32, b:&f32 | *a < *b)
            .with_init(vec![81.5, 4.5,4.,1.,45.,22.,11.])
            .into_iter()
            .collect::<Vec<_>>());

        dbg!(TopIter::new(3)
            .with_init(vec![81.5, 4.5,4.,1.,45.,22.,11.])
            .with_selector(|a, b| *a > *b)
            .into_iter()
            .collect::<Vec<_>>());
    }

    #[test]
    fn ord()
    {
        dbg!(TopIter::new(5)
            .with_init(vec![81,5, 4,5,4,1,45,22,1,5,97,5,877,12,0])
            .into_iter()
            .collect::<Vec<_>>());
    }
}