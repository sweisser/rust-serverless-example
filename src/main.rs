use lambda_runtime::{Context, error::HandlerError};
use lambda_http::{lambda, Request, IntoResponse, RequestExt};
use serde_derive::{Serialize, Deserialize};
use serde_json::json;
use rust_serverless_example::{ compute_holidays, Holidays };

#[derive(Debug, Deserialize, Default)]
struct Args {
    #[serde(default)]
    year: i32,
}

#[derive(Serialize)]
struct MyResponse {
    version: String,
    holidays: Holidays,
}

fn main() {
    lambda!(handler)
}

fn handler(
    request: Request,
    _ctx: Context
) -> Result<impl IntoResponse, HandlerError> {
    let args: Args = request.payload()
        .unwrap_or_else(|_parse_err| None)
        .unwrap_or_default();
    let r = MyResponse {
        version: String::from("1.0"),
        holidays: compute_holidays(args.year),
    };

    Ok(json!(r))
}
