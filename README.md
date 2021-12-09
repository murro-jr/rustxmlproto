# rustxmlproto

This is the main code repository for rustxmlproto. This is intended to be used in
constructing an XML prototype for rustlang objects such as structs, traits and enums.
One can create and XML file with elements that models a rust object. It can be
added to a project build process. See [demo](https://github.com/murro-jr/rustxmlproto/tree/master/demo/) project for an example.

## Usage

**XML Format**

```
<prototype name="Foobar" class="struct" visibility="crate">
    <includes>
        <within name="foobar_within" scope="all/>
        <extern name="foobar_external" objects="Object1, Object2"/>
    </includes>
    <members>
        <String name="item" visibility="external"/>
        <u32 name="price"/>
    </members>
    <functions>
        <generic name="addDiscount">
            <parameters>
                <u32 name="discount"/>
            </parameters>
        </generic>
    </functions>
</prototype>
```
**Rust module**

```
import_proto!("foobar");

fn main() {
    let foobar_obj = Foobar::new(
      String::from("foobar"),
      123456789
    );
}
```

## Contributing
Contributions are very much welcome. Please make sure tests are updated and commits are verified.
