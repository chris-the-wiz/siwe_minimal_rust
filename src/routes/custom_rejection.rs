/// Custom Rejection
/// 
use warp::{Rejection, Reply};
use std::convert::Infallible;

#[derive(Debug)]
pub (crate) struct CustomRejection {
  pub (crate)reason: String,
}
impl warp
::reject::Reject for CustomRejection {}  



pub (crate) async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
  
  if let Some(custom_rejection) = err.find::<CustomRejection>() {
      // Handle the custom rejection and return an appropriate response
      let response = format!("Custom Rejection: {}", custom_rejection.reason);
      Ok(warp::reply::with_status(response, warp::http::StatusCode::BAD_REQUEST))
  } else {
      // Handle other types of rejections or return a generic response
      Ok(warp::reply::with_status("Internal Server Error".to_string(), warp::http::StatusCode::INTERNAL_SERVER_ERROR))
  }
}
