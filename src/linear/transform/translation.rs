pub trait Translation<M> {
    fn translate(&self, value: &mut M);
}
