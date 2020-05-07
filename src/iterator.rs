pub trait Until<T> {
    fn until(&mut self, check: impl Fn(&T) -> bool) -> usize;
}

impl<T, I> Until<T> for std::iter::Peekable<I>
where
    I: Iterator<Item = T>,
{
    fn until(&mut self, check: impl Fn(&T) -> bool) -> usize {
        let mut cpt = 0;
        while let Some(el) = self.peek() {
            if check(el) {
                break;
            }
            self.next();
            cpt += 1;
        }
        cpt
    }
}
