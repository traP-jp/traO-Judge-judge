pub trait Alert {
    async fn send_alert<MessageType: std::fmt::Display + Clone>(
        &self,
        message: &MessageType,
    );
}

pub trait StructuralAlert<BodyType: serde::Serialize + Clone> {
    async fn send_alert(
        &self,
        body: &BodyType,
    );
}