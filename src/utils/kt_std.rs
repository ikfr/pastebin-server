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

trait IterExt<T> {
    fn on_each(&self, f: impl Fn(&T)) -> &Self;
}

impl<T> IterExt<T> for Vec<T> {
    fn on_each(&self, f: impl Fn(&T)) -> &Self {
        self.also_ref(|v| v.iter().for_each(|e| f(e)))
    }
}
