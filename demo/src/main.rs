fn main() {
    println!("Demo run for TestProto...");

    let proto = TestProto::new(
        String::from("Joe Biden"),
        String::from("White House"),
        123456789,
    );

    assert_eq!(proto.clone().get_name(), String::from("Joe Biden"));
    println!("TEST CASE 1: proto.name = 'Joe Biden', passed");

    assert_eq!(proto.clone().get_current_address(), String::from("White House"));
    println!("TEST CASE 2: proto.current_address = 'White House', passed");

    assert_eq!(proto.clone().get_id(), 123456789);
    println!("TEST CASE 3: proto.id = '123456789', passed");

    let proto = proto.set_name(String::from("Donald Trump"));
    assert_eq!(proto.clone().get_name(), String::from("Donald Trump"));
    println!("TEST CASE 4: proto.name = 'Donald Trump', passed");

    let proto = proto.set_current_address(String::from("Washington, DC"));
    assert_eq!(proto.clone().get_current_address(), String::from("Washington, DC"));
    println!("TEST CASE 5: proto.current_address = 'Washington, DC', passed");

    let proto = proto.set_id(987654321);
    assert_eq!(proto.clone().get_id(), 987654321);
    println!("TEST CASE 6: proto.id = '987654321', passed");
}
