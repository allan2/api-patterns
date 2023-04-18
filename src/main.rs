#![allow(dead_code)]

use fluent::FluentBuilderClient;
use simple::{SayHelloInput, SimpleClient};
mod fluent;
mod simple;

fn main() {
    // mutable input struct is a simpler implementation
    let client = SimpleClient::new();
    let input = SayHelloInput {
        first_name: "Foo".to_owned(),
        last_name: "Bar".to_owned(),
        ..SayHelloInput::default()
    };
    client.say_hello(input);

    // fluent builder has pretty function chaining
    let client = FluentBuilderClient::new();
    client.say_hello().first_name("Foo").last_name("Bar").send();

    // fluent builder again
    let mut builder = client.say_hello();
    builder.first_name("Foo");
    builder.last_name("Bar");
    builder.send();
}
