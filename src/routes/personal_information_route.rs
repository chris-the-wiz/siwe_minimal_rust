
use warp::{ Rejection,  reply::Html };
use warp_sessions::{ MemoryStore, SessionWithStore};//,  SameSiteCookieOption };
use std::sync::{ Arc, RwLock};
use siwe:: Message; //, VerificationOpts };





pub(crate) async fn personal_information_handler(mut session_with_store:SessionWithStore<MemoryStore>) 
->  Result<(Html<String>, SessionWithStore<MemoryStore>), Rejection> {



  //session.get_raw("key").is_none());

  let shared_session = Arc::new(RwLock::new(session_with_store.session.clone()));

  let siwe_from_session:String = shared_session
    .read()
    .unwrap()
    .get("siwe").or_else(
      || Some(String::from(""))
    )
    .unwrap();

  if siwe_from_session.is_empty() {
    return Ok((
        warp::reply::html(String::from("You are not authenticated")), 
        session_with_store
      ))
  }
     
     
  println!("body: {}", siwe_from_session);
  let siwe_parsed:Message = siwe_from_session.parse().unwrap_or_else(|err| { panic!("error parsing message json: {}", err) });

  //let _t = siwe_parsed.address;
  let address_hex:String =   eip55::checksum(hex::encode(siwe_parsed.address).as_str());

  session_with_store.session = Arc::try_unwrap(shared_session)
  .unwrap()
  .into_inner()
  .unwrap(); 
  
  Ok::<_, Rejection>((  
      warp::reply::html(format!("{}{}", "You are authenticated and your address is: ", address_hex)),
      session_with_store 
  ))
}
  

