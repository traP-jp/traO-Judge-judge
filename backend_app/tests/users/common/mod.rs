use axum::{
    body::Body,
    http::{request, Request},
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

pub fn remove_field(json: &mut serde_json::Value, ignore_fields: &Vec<&str>) {
    match json {
        serde_json::Value::Object(obj) => {
            for field in ignore_fields {
                obj.remove(*field);
            }
        }
        _ => {
            panic!("Invalid JSON format");
        }
    }
}
