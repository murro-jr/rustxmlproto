#[macro_export]
macro_rules! import_proto {
    ($value:expr) => {
        include!(concat!(env!("PROTO_DIR"),concat!("/", $value, ".rs")));
    }
}
