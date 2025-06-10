mod v1;

use bytes::Bytes;

struct Msg {
    type_code: u8,
    type_flag: u8,
    type_info: Option<Bytes>,
    value: Bytes,
}
