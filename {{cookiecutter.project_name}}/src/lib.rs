use lambda_http::{IntoResponse, Request, RequestExt};
use lambda_runtime::{Context, error::HandlerError};

pub fn hello(
    request: Request,
    _ctx: Context
) -> Result<impl IntoResponse, HandlerError> {
    Ok(format!(
        "hello from rust, {}!",
        request
            .query_string_parameters()
            .get("name")
            .unwrap_or_else(|| "stranger")
    ))
}

pub fn goodbye(
    request: Request,
    _ctx: Context
) -> Result<impl IntoResponse, HandlerError> {
    Ok(String::from("goodbye from rust!"))
}