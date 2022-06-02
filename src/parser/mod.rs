use std::marker::PhantomData;

pub mod declaration;
pub mod rule;

#[derive(Debug, Default, Clone)]
pub struct ParserOptions<'i> {
    // TODO
    p: PhantomData<&'i Self>,
}
