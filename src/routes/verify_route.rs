use warp::{ Rejection,   reply::Html,   };
use warp_sessions::{ MemoryStore, SessionWithStore,  SameSiteCookieOption };
use std::sync::{ Arc, RwLock};
use serde::{ Deserialize, Serialize };
use siwe::{ Message, VerificationOpts };
use hex::FromHex;
use time::{format_description::well_known::Rfc3339,  OffsetDateTime };//, Time};




#[derive(Serialize, Deserialize, Debug)] 
  pub struct VerifyRequest {
    message: String,
    signature: String, //nature
  }
  

use  crate::routes::custom_rejection::CustomRejection;



fn session_read_nonce(mut session_with_store:SessionWithStore<MemoryStore> )->String{
   // /////////////////////session part
 
   session_with_store.cookie_options = warp_sessions::CookieOptions {
    cookie_name: "siwe_minimal",
    cookie_value: None,
    max_age:Some(3600),
    domain: None,
    path: None,
    secure: true,
    http_only: true,
    same_site: Some(SameSiteCookieOption::Strict),
  };

  let  shared_session = Arc::new(RwLock::new(session_with_store.session));


  let read_guard = shared_session.read().unwrap();
   let nonce: String = match read_guard.get("nonce") {
       Some(val) => val,
       None => "".to_string(),
   };
   let _b_pre: String = match read_guard.get("siwe") {
       Some(val) => val,
       None => "".to_string(),
   };
  
   drop(read_guard);
   
   nonce


  
}



pub async fn verify_handler(body: warp::hyper::body::Bytes, session_with_store:SessionWithStore<MemoryStore> ) 
-> Result<(Html<String>, SessionWithStore<MemoryStore>), Rejection>  {


  let session_nonce = session_read_nonce(session_with_store.clone());
 
   
  /////////////////////////////////
    
  let param = String::from_utf8_lossy(&body).to_string();
  println!("param: {}", param);


  let sign_request: VerifyRequest = serde_json::from_str(param.as_str()).unwrap_or_else(|err| { panic!("error parsing json: {}", err) });
  let message: Message = sign_request.message.parse().unwrap_or_else(|err| { panic!("error parsing message json: {}", err) });
 
  
  
  let mut decoded: [u8; 65] = [0; 65];
  hex::decode_to_slice(&sign_request.signature.trim_start_matches("0x"), &mut decoded).expect("Decoding failed");
  let signature = <[u8; 65]>::from_hex(sign_request.signature.trim_start_matches("0x")).unwrap();
  let nonce = message.nonce.as_str();

 

  let time = message.issued_at.clone().to_string();
  let uri = message.uri.as_str().strip_prefix("http://").unwrap_or_else(|| message.uri.as_str().strip_prefix("https://").unwrap());

  let verification_opts = VerificationOpts {
    domain: Some(uri.parse().unwrap()),
    nonce:  Some(session_nonce),
    timestamp: Some(OffsetDateTime::parse(&time, &Rfc3339).unwrap()),
    ..Default::default()
  };


  if let Err(e) = message.verify(&signature, &verification_opts).await
  {

    println!("Error: {}", e);
    
    return Err(warp::reject::custom(CustomRejection { reason:e.to_string() }));
   
      
  }

  


  let shared_session = Arc::new(RwLock::new(session_with_store.session.clone()));

   
   let msgstr = message.to_string();
   let mut write_guard = shared_session.write().unwrap();
   write_guard.insert("nonce", nonce.to_string())
   .or_else(|_er| {write_guard.insert("nonce", "")})
   .unwrap_or_else(|er| {panic!("error: {}", er)});
   
  

   write_guard.insert("siwe", msgstr)
   .or_else(|_er| {write_guard.insert("siwe", "")})
   .unwrap_or_else(|er| {panic!("error: {}", er)});
  
   drop(write_guard);  // Drop the write guard once you're done with it


   Ok::<_, Rejection>(
    (
      warp::reply::html("Verification Done".to_string()),
      session_with_store
    )
  )
  





 
}
