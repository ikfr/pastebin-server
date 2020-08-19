pub trait KtStd {
    fn let_ref<R>(&self, block: fn(&Self) -> R) -> R {
        block(self)
    }

    fn let_mut<R>(&mut self, block: fn(&mut Self) -> R) -> R {
        block(self)
    }

    fn let_owned<R>(self, block: fn(Self) -> R) -> R where Self: Sized {
        block(self)
    }

    fn also_ref(&self, block: fn(&Self)) -> &Self {
        block(self);
        self
    }

    fn also_mut(&mut self, block: fn(&mut Self)) -> &mut Self {
        block(self);
        self
    }
}

impl <T> KtStd for T {}
