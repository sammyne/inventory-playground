//use inventory::submit;

pub struct Hello {
    pub src: String,
}

impl Hello {
    pub fn new(src: &str) -> Self {
        Self {
            src: src.to_string(),
        }
    }
}

inventory::collect!(Hello);

inventory::submit! {
    Hello::new("hello")
}
