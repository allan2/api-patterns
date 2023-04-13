pub struct FluentBuilderClient;

impl FluentBuilderClient {
    pub fn new() -> Self {
        Self
    }

    pub fn say_hello(&self) -> SayHelloBuilder {
        SayHelloBuilder::default()
    }
}

#[derive(Default)]
pub struct SayHelloBuilder {
    first_name: String,
    last_name: String,
}

impl SayHelloBuilder {
    pub fn first_name(&mut self, input: &str) -> &mut Self {
        self.first_name = input.to_owned();
        self
    }
    pub fn last_name(&mut self, input: &str) -> &mut Self {
        self.last_name = input.to_owned();
        self
    }

    pub fn send(&self) {
        println!("Hello {} {}", self.first_name, self.last_name);
    }
}
