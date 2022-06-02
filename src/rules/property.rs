use std::marker::PhantomData;

pub struct PropertyRule<'i> {
    p: PhantomData<&'i Self>,
}