pub struct SimpleClient;

impl SimpleClient {
    pub fn new() -> Self {
        Self
    }

    pub fn say_hello(&self, input: SayHelloInput) {
        println!(
            "Hello {} {} {}",
            input.first_name, input.last_name, input.title
        )
    }
}

pub struct SayHelloInput {
    pub first_name: String,
    pub last_name: String,
    pub title: String,
}

impl Default for SayHelloInput {
    fn default() -> Self {
        Self {
            first_name: String::new(),
            last_name: String::new(),
            title: "the Great".to_owned(),
        }
    }
}
