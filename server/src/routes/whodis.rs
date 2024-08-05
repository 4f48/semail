use axum::Json;
use serde::Serialize;
use serde_json::Value;

#[derive(Serialize)]
struct InstanceInfo {
    version: &'static str,
    instance_name: &'static str,
    instance_provider: &'static str,
    provider_email: &'static str,
    public_key: &'static str,
}

pub async fn main() -> Json<Value> {
    let instance_info = InstanceInfo {
        version: "1.0.0-alpha.1",
        instance_name: "development",
        instance_provider: "0x4f48",
        provider_email: "0x4f48@proton.me",
        public_key: "",
    };

    Json(serde_json::to_value(instance_info).unwrap())
}
