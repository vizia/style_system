use std::marker::PhantomData;

#[derive(Debug, Clone, PartialEq)]
pub struct PropertyRule<'i> {
    p: PhantomData<&'i Self>,
}
