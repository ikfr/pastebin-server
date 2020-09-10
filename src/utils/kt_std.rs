pub trait KtStd {
    fn let_ref<R>(&self, block: impl FnOnce(&Self) -> R) -> R {
        block(self)
    }

    fn let_mut<R>(&mut self, mut block: impl FnMut(&mut Self) -> R) -> R {
        block(self)
    }

    fn let_owned<R>(self, block: impl FnOnce(Self) -> R) -> R where Self: Sized {
        block(self)
    }

    fn also_ref(&self, block: impl FnOnce(&Self)) -> &Self {
        block(self);
        self
    }

    fn also_mut(&mut self, mut block: impl FnMut(&mut Self)) -> &mut Self {
        block(self);
        self
    }
}

impl <T> KtStd for T {}
