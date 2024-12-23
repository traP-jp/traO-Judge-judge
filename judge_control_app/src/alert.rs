pub trait Alert<BodyType = ()> {
    async fn send_alert<MessageType: std::fmt::Display>(
        &self,
        message: MessageType,
        body: BodyType,
    );
}
