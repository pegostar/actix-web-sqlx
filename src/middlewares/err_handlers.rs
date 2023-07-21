use actix_web::dev::ServiceResponse;
use actix_web::http::StatusCode;
use actix_web::middleware::{ErrorHandlerResponse, ErrorHandlers};

pub fn err_handlers<B: 'static>() -> ErrorHandlers<B> {
    ErrorHandlers::new()
        .handler(StatusCode::INTERNAL_SERVER_ERROR, internal_error)
        .handler(StatusCode::NOT_FOUND, not_found)
}


pub fn internal_error<B>(res: ServiceResponse<B>) -> actix_web::Result<ErrorHandlerResponse<B>> {
    // Get the error message and status code
    let error_message = "An error occurred";
    // Destructures ServiceResponse into request and response components
    let (req, res) = res.into_parts();
    // Create a new response with the modified body
    let res = res.set_body(error_message).map_into_boxed_body();
    // Create a new ServiceResponse with the modified response
    let res = ServiceResponse::new(req, res).map_into_right_body();
    Ok(ErrorHandlerResponse::Response(res))
}

pub fn not_found<B>(res: ServiceResponse<B>) -> actix_web::Result<ErrorHandlerResponse<B>> {
    // Get the error message and status code
    let error_message = "Not found";
    // Destructures ServiceResponse into request and response components
    let (req, res) = res.into_parts();
    // Create a new response with the modified body
    let res = res.set_body(error_message).map_into_boxed_body();
    // Create a new ServiceResponse with the modified response
    let res = ServiceResponse::new(req, res).map_into_right_body();
    Ok(ErrorHandlerResponse::Response(res))
}
