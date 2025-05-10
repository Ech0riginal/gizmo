mod v2;
// mod v3;
// mod v3g;

mod de;
mod placeholder;
// #[cfg(test)]
// pub(self) mod tests;

pub use v2::V2;
// pub use v3::V3;
// pub use v3g::V3g;

/// Encompasses GraphSON v2, v3, and our custom types. Our custom types will always be the last in
/// this list.
#[allow(unused)]
pub(crate) mod types {
    pub(crate) use super::v2::types as v2;
    // pub(crate) use super::v3::types as v3;
    // pub(crate) use super::v3g::types as v3g;
}
