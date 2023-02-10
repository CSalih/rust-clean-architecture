pub trait Presenter<T> {
    fn success(&mut self, data: T);
    fn error(&mut self, error: String);
}