pub struct SimpleClient;

impl SimpleClient {
    pub fn new() -> Self {
        Self
    }

    pub fn say_hello(&self, input: SayHelloInput) {
        println!("Hello {} {}", input.first_name, input.last_name)
    }
}

#[derive(Default)]
pub struct SayHelloInput {
    pub first_name: String,
    pub last_name: String,
}
