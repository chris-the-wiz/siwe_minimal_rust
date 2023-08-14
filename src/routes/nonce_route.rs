use warp::{ Rejection,  reply::Html };

use warp_sessions::{ MemoryStore, SessionWithStore };
use std::sync::{ Arc, RwLock};
use siwe;




pub(crate) async  fn nonce_handler(mut session_with_store:SessionWithStore<MemoryStore>) 
  ->  Result<(Html<String>, SessionWithStore<MemoryStore>), Rejection> {
  

  let siwenonce = siwe::generate_nonce();

  // /////////////////////session part
   
  
  let shared_session = Arc::new(RwLock::new(session_with_store.session));

  shared_session
      .write()
      .unwrap()
      .insert("nonce", siwenonce.clone() )
      .unwrap();

  session_with_store.session = Arc::try_unwrap(shared_session)
      .unwrap()
      .into_inner()
      .unwrap(); 



  

      Ok::<_, Rejection>(
        (
          warp::reply::html(siwenonce),
          session_with_store
        )
      )
      

}