pub trait Generator<I> {
    type Output;

    fn generate(input: I) -> Self::Output;
}
