pub struct FluentBuilderClient;

impl FluentBuilderClient {
    pub fn new() -> Self {
        Self
    }

    pub fn say_hello(&self) -> SayHelloBuilder {
        SayHelloBuilder::default()
    }
}

pub struct SayHelloBuilder {
    pub first_name: String,
    pub last_name: String,
    pub title: String,
}

impl Default for SayHelloBuilder {
    fn default() -> Self {
        Self {
            first_name: String::new(),
            last_name: String::new(),
            title: "the Great".to_owned(),
        }
    }
}

impl SayHelloBuilder {
    pub fn first_name(&mut self, s: &str) -> &mut Self {
        self.first_name = s.to_owned();
        self
    }
    pub fn last_name(&mut self, s: &str) -> &mut Self {
        self.last_name = s.to_owned();
        self
    }
    pub fn title(&mut self, s: &str) -> &mut Self {
        self.title = s.to_owned();
        self
    }

    pub fn send(&self) {
        println!(
            "Hello {} {} {}",
            self.first_name, self.last_name, self.title
        );
    }
}
