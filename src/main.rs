
use warp::Filter;
use warp_sessions::{ MemoryStore, SameSiteCookieOption};


mod routes;
use routes::{   nonce_route::nonce_handler, verify_route::verify_handler, custom_rejection::handle_rejection, personal_information_route::personal_information_handler}; //,


#[cfg(test)]
pub mod tests;


#[tokio::main]
async fn main() {
    let   session_store = MemoryStore::new();
   

    
    let cors = warp::cors()
    .allow_any_origin()
    .allow_headers(vec!["Access-Control-Allow-Origin", "Origin", "Accept", "X-Requested-With", "Content-Type","Access-Control-Allow-Credentials", "Authorization"])
    .allow_credentials(true)
    .allow_methods(vec!["GET", "POST", "DELETE"]);



    
    let cookie_options = warp_sessions::CookieOptions {
      cookie_name: "siwe_minimal",
      cookie_value: None,
      max_age:Some(3600),
      domain: None,
      path: None,
      secure: true,
      http_only: true,
      same_site: Some(SameSiteCookieOption::Strict),
    };


    let nonce_route 
    = warp::path("nonce")
    .and(warp::get())
    .and(warp_sessions::request::with_session(session_store.clone(), Some(cookie_options.clone())))
    .and_then(nonce_handler)
    .untuple_one()
    .and_then(warp_sessions::reply::with_session)
    .with(&cors);
 

    


    let verify_route 
    = warp::path("verify")
    .and(warp::post())
    .and(warp::body::bytes())
    .and(warp_sessions::request::with_session(session_store.clone(), Some(cookie_options.clone())))
    .and_then( verify_handler)
    .untuple_one()
    .and_then(warp_sessions::reply::with_session)
    .with(&cors); 

      
    let personal_information_route 
    = warp::path("personal_information")
    .and(warp::get())
    .and(warp_sessions::request::with_session(session_store.clone(), Some(cookie_options.clone())))
    .and_then(personal_information_handler)
    .untuple_one()
    .and_then(warp_sessions::reply::with_session)
    .with(&cors);

    

    // Start the server
    let port = 3000;
    println!("starting server listening on ::{}", port);
    warp::serve( 
      personal_information_route
      .or(nonce_route)
      .or(verify_route) 
      .recover(handle_rejection)    
    ).run(([0, 0, 0, 0], port)).await;

}



