use crate::{Context, View};

pub fn from_fn<F, M>(f: F) -> FromFn<F>
where
    F: FnMut(&mut Context<M>),
{
    FromFn { f }
}

pub struct FromFn<F> {
    f: F,
}

impl<T, M, F> View<T, M> for FromFn<F>
where
    F: FnMut(&mut Context<M>),
{
    type Element = ();

    fn build(&mut self, cx: &mut Context<M>, _tree: &mut T) -> Self::Element {
        (self.f)(cx)
    }

    fn rebuild(&mut self, cx: &mut Context<M>, _tree: &mut T, _state: &mut Self::Element) {
        (self.f)(cx)
    }
}
