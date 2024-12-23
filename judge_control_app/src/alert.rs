pub trait Alert {
    async fn send_alert<T: std::fmt::Display>(&self, message: T);
}
