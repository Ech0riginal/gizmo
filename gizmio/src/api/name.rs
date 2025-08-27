/// Gives an 'object' a name for surfacing errors and debugging.
pub trait Named: std::fmt::Debug {
    #[allow(nonstandard_style)]
    const name: &'static str;
}
