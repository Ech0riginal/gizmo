/// Gives an 'object' a name for surfacing errors and debugging.
pub trait Named {
    #[allow(nonstandard_style)]
    const name: &'static str;
}
