use clap::Parser;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Parser)]
pub struct Cli {
    pub first_arg: Option<String>,
    pub second_arg: Option<String>,
    pub third_arg: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct ExtractVariable {
    pub key_path: String,
    pub variable_name: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct HttpRequest {
    pub method: String,
    pub url: String,
    pub headers: Vec<String>,
    pub body_type: Option<String>,
    pub body: Option<Value>,
    pub extract_variables: Option<Vec<ExtractVariable>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GlobalVariable {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct HttpResponse {
    pub method: String,
    pub url: String,
    pub status_code: u16,
    pub json_data: Option<Value>,
    pub text_data: Option<String>,
}
