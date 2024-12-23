pub trait Alert {
    fn send_alert<T: std::fmt::Display>(&self, message: T);
}
