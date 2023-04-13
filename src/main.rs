use fluent::FluentBuilderClient;
use simple::{SayHelloInput, SimpleClient};
mod fluent;
mod simple;

fn main() {
    // mutable input struct is simpler
    let client = SimpleClient::new();
    let mut input = SayHelloInput::default();
    input.first_name = "Foo".to_owned();
    input.last_name = "Bar".to_owned();
    client.say_hello(input);

    // fluent builder allows for function chaining
    let client = FluentBuilderClient::new();
    client.say_hello().first_name("Foo").last_name("Bar").send();

    // alternative with fluent builder
    let mut builder = client.say_hello();
    builder.first_name("Foo");
    builder.last_name("Bar");
    builder.send();
}
