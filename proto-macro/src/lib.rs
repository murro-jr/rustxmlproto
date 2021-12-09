#[macro_export]
macro_rules! import_proto {
    ($value:expr) => {
        include!(concat!(env!("OUT_DIR"),concat!("/", $value, ".rs")));
    }
}