use std::cmp::Reverse;

pub trait ToReverse<T = Self> {
    fn rev(self) -> Reverse<T>;
}

impl<T> ToReverse for T
where
    T: PartialOrd,
{
    fn rev(self) -> Reverse<Self> {
        Reverse(self)
    }
}
