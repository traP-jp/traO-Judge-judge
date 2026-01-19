use axum::{
    body::Body,
    http::{Request, request},
};

pub mod check;

pub trait RequestBuilderExt {
    fn json(self, json: serde_json::Value) -> Request<Body>;
}

impl RequestBuilderExt for request::Builder {
    fn json(self, json: serde_json::Value) -> Request<Body> {
        self.header("Content-Type", "application/json")
            .body(Body::from(json.to_string()))
            .unwrap()
    }
}
