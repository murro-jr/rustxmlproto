mod prototype;

use crate::prototype::Prototype;
use serde_xml_rs::{from_str, to_string};

fn main() {
    let src = r#"<Prototype>
                    <name>TestProto</name>
                    <class>enum</class>
                    <functions></functions>
                </Prototype>"#;

    let prototype: Prototype = from_str(src).unwrap();
    println!("{:?}", prototype)
}
