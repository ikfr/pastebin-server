pub trait Let {
    fn let_imut<R, F>(&self, block: F) -> R
    where
        F: FnOnce(&Self) -> R;

    fn let_mut<R, F>(&mut self, block: F) -> R
    where
        F: FnMut(&mut Self) -> R;

    fn let_owned<R, F>(self, block: F) -> R
    where
        Self: Sized,
        F: FnOnce(Self) -> R;
}

impl<T> Let for T {
    fn let_imut<R, F>(&self, block: F) -> R
    where
        F: FnOnce(&Self) -> R,
    {
        block(self)
    }

    fn let_mut<R, F>(&mut self, mut block: F) -> R
    where
        F: FnMut(&mut Self) -> R,
    {
        block(self)
    }

    fn let_owned<R, F>(self, block: F) -> R
    where
        Self: Sized,
        F: FnOnce(Self) -> R,
    {
        block(self)
    }
}
