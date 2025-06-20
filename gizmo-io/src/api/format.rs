pub trait Format {
    #[allow(nonstandard_style)]
    const mime: &'static str;

    type Serial: super::DeserializeExt;
}
