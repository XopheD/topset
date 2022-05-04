
use std::cmp::Ordering;
use std::marker::PhantomData;
use crate::TopSet;

/// An iterator builder over the N greatest items of a given set
#[derive(Clone)]
pub struct TopIter { count: usize }

#[derive(Clone)]
pub struct TopIterI<I:IntoIterator> { count: usize, init: I }

#[derive(Clone)]
pub struct TopIterC<X,C:Fn(&X, &X)->Ordering> { count: usize, cmp: C, item: PhantomData<X> }

#[derive(Clone)]
pub struct TopIterIC<I:IntoIterator,C:Fn(&I::Item, &I::Item)->Ordering> { top: TopSet<I::Item,C> }


impl Default for TopIter
{
    fn default() -> Self { Self::new(1) }
}

impl TopIter
{
    /// A Top N iterator
    pub fn new(n: usize) -> Self {
        assert!( n > 0 );
        Self { count: n }
    }

    /// Specify the initial set of items to scan
    pub fn with_init<I:IntoIterator>(self, init:I) -> TopIterI<I>
    {
        TopIterI { count: self.count, init }
    }

    /// Specify the comparison to use
    pub fn with_compare<X, C:Fn(&X, &X)->Ordering>(self, cmp:C) -> TopIterC<X,C>
    {
        TopIterC { count: self.count, cmp, item: PhantomData::default() }
    }
}


impl<I:IntoIterator> TopIterI<I>
{
    pub fn with_compare<C:Fn(&I::Item, &I::Item)->Ordering>(self, cmp:C) -> TopIterIC<I,C>
    {
        TopIterIC { top: TopSet::with_init(self.count, self.init, cmp) }
    }
}


impl<X,C:Fn(&X, &X)->Ordering> TopIterC<X,C>
{
    pub fn with_init<I:IntoIterator<Item=X>>(self, init:I) -> TopIterIC<I,C>
    {
        TopIterIC { top: TopSet::with_init(self.count, init, self.cmp) }
    }
}


impl<I> IntoIterator for TopIterI<I>
    where
        I: IntoIterator,
        I::Item : Ord
{
    type Item = I::Item;
    type IntoIter = <Vec<I::Item> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.with_compare(<I::Item as Ord>::cmp).into_iter()
    }
}

impl <I:IntoIterator,C:Fn(&I::Item, &I::Item)->Ordering> IntoIterator for TopIterIC<I,C>
{
    type Item = I::Item;
    type IntoIter = <Vec<I::Item> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.top.into_iter()
    }
}


#[cfg(test)]
mod tests {
    use crate::TopIter;

    #[test]
    fn cost()
    {
        dbg!(TopIter::new(5)
            .with_compare(|a:&f32, b:&f32 | b.partial_cmp(a).unwrap())
            .with_init(vec![81.5, 4.5,4.,1.,45.,22.,11.])
            .into_iter()
            .collect::<Vec<_>>());

        dbg!(TopIter::new(5)
            .with_init(vec![81.5, 4.5,4.,1.,45.,22.,11.])
            .with_compare(|a, b| a.partial_cmp(b).unwrap())
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