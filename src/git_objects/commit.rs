use super::git_object::GitObject;

pub struct Commit {
    content: Vec<u8>,
}

impl Commit {
    pub fn new() -> Self {
        todo!()
    }
}

impl GitObject for Commit {
    fn pretty_print(&self) {
        println! {"{}",  String::from_utf8(self.content.clone()).unwrap()}
    }
    fn format_object(&self) -> String {
        format!(
            "commit {}\0{}",
            self.content.len(),
            String::from_utf8(self.content.clone()).unwrap()
        )
    }
}
