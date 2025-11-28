pub struct Fake();

impl Fake {
    pub fn new() -> Self {
        Fake {}
    }

    pub fn list_all() -> Vec<String> {
        vec!["fake1".to_string(), "fake2".to_string()]
    }

    pub fn get() -> String {
        "This is a fake entity".to_string()
    }
}
