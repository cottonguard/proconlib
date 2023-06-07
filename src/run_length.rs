pub fn run_length_comp<I: IntoIterator>(iter: I) -> RunLengthComp<I::IntoIter>
where
    I::Item: Eq,
{
    RunLengthComp {
        inner: iter.into_iter(),
        last: None,
    }
}
pub struct RunLengthComp<I: Iterator> {
    inner: I,
    last: Option<I::Item>,
}
impl<I: Iterator> Iterator for RunLengthComp<I>
where
    I::Item: Eq,
{
    type Item = (I::Item, usize);
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(last) = self.last.take().or_else(|| self.inner.next()) {
            let mut len = 1;
            while let Some(v) = self.inner.next() {
                if v != last {
                    self.last = Some(v);
                    return Some((last, len));
                }
                len += 1;
            }
            Some((last, len))
        } else {
            None
        }
    }
}
