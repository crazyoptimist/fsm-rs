pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

trait State {}
