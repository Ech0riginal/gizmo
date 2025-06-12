use crate::Object;

pub type Uuid = ::uuid::Uuid;
impl Object for Uuid {
    const name: &'static str = "Uuid";
}
