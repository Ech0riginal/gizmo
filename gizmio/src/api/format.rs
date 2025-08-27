use crate::DeserializeExt;
use crate::api::Bytable;

pub trait Format: Clone + Send + Sync + 'static {
    #[allow(nonstandard_style)]
    const mime: &'static str;

    type Serial: DeserializeExt + Bytable;
}
