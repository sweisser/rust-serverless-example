use lambda_runtime::{Context, error::HandlerError};
use lambda_http::{lambda, Request, IntoResponse, RequestExt};
use serde_derive::{Serialize, Deserialize};
use serde_json::json;
use rust_serverless_example::{ compute_holidays, Holidays };
use std::fmt;

fn main() {
    lambda!(handler)
}

fn handler(
    request: Request,
    _ctx: Context
) -> Result<impl IntoResponse, HandlerError> {
    match request.payload::<MyRequest>() {
        Ok(req) => {
            match req {
                Some(req_ok) => {
                    // All parsing went okay.
                    let r = MyResponse {
                        version: String::from("1.0"),
                        holidays: compute_holidays(req_ok.year),
                    };

                    Ok(json!(r))
                },
                None => {
                    Err(HandlerError::new(MyError::new("Error deserializing the request. Maybe some fields missing?")))
                }
            }
        },
        Err(_e) => {
            Err(HandlerError::new(MyError::new("Error parsing the request (must be POST with Content-Type: application/json).")))
        }
    }
}

#[derive(Debug, Deserialize, Default)]
struct MyRequest {
    #[serde(default)]
    year: i32,
}

#[derive(Serialize)]
struct MyResponse {
    version: String,
    holidays: Holidays,
}

#[derive(Serialize, Debug, Clone)]
struct MyError {
    errormessage: String,
}

impl MyError {
    pub fn new(msg: &str) -> MyError {
        MyError {
            errormessage: String::from(msg)
        }
    }
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.errormessage)
    }
}

impl std::error::Error for MyError {}

impl lambda_runtime::error::LambdaErrorExt for MyError {
    fn error_type(&self) -> &str {
        "MyError"
    }
}