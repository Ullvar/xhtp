use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(Debug, Deserialize)]
pub struct Operation {
    pub summary: Option<String>,
    pub description: Option<String>,
    pub operation_id: Option<String>,
    pub parameters: Option<Vec<Parameter>>,
    pub request_body: Option<RequestBody>,
    pub responses: BTreeMap<String, Response>,
}

#[derive(Debug, Deserialize)]
pub struct Parameter {
    pub name: String,
    pub description: Option<String>,
    pub required: Option<bool>,
    pub schema: Option<Schema>,
}

#[derive(Debug, Deserialize)]
pub struct RequestBody {
    pub description: Option<String>,
    pub content: BTreeMap<String, MediaType>,
}

#[derive(Debug, Deserialize)]
pub struct MediaType {
    pub schema: Option<Schema>,
}

#[derive(Debug, Deserialize)]
pub struct Response {
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct Schema {}

#[derive(Debug, Deserialize)]
pub struct PathItem {
    pub summary: Option<String>,
    pub description: Option<String>,
    pub get: Option<Operation>,
    pub put: Option<Operation>,
    pub post: Option<Operation>,
    pub delete: Option<Operation>,
}

#[derive(Debug, Deserialize)]
pub struct Info {
    pub title: String,
    pub version: String,
}

#[derive(Debug, Deserialize)]
pub struct OpenAPI {
    pub openapi: String,
    pub info: Info,
    pub paths: BTreeMap<String, PathItem>,
}
