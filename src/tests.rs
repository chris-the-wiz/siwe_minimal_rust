



use std::sync::Arc;


use test_log::test;

use reqwest::{  Client, cookie::Jar, Error};


use ethers_core::rand::thread_rng;
use ethers_signers::{LocalWallet, Signer};
use std::collections::HashMap;





fn setup_client() ->reqwest::Client{
        
    let jar = Arc::new(Jar::default());

    let client = reqwest::Client::builder()
    .cookie_provider(Arc::clone(&jar))
    .build()
    .unwrap();
    client
}


struct StringStruct{
    s: String,
}

impl std::process::Termination for StringStruct{
    fn report(self) -> std::process::ExitCode{std::process::ExitCode::SUCCESS}

}




#[tokio::main]
#[test]
async fn nonce_test()-> Result<(), Error>
{
    let client = setup_client();
    let nonce_resp = nonce_inner(&client).await?;
    assert_eq!(nonce_resp.s.len(),17);
    Ok(())
}



async fn nonce_inner(client:&Client)-> Result<StringStruct, Error>
{
 
    let  response = client.get("http://127.0.0.1:3000/nonce").send().await?;
    let nonce_resp = response.text().await?;  
  
    let out: StringStruct = StringStruct{s: nonce_resp};
    Ok(out) 
}



#[tokio::main]
#[test]
async fn verify_test() ->  Result<(),Error>
{
    let client = setup_client();
    let wallet = LocalWallet::new(&mut thread_rng());
    let nonce_resp  = nonce_inner(&client).await?.s;
    assert_eq!(nonce_resp.len(),17);
    let verify_resp = verify_inner(&client, &nonce_resp,&wallet).await?.s;
    assert_eq!(verify_resp,"Verification Done".to_string());
    Ok(())
}

#[tokio::main]
#[test]
async fn verify_test_no_nonce() ->  Result<(),Error>
{
    let client = setup_client();
    let wallet = LocalWallet::new(&mut thread_rng());

    
    let verify_resp = verify_inner(&client, &format!("12345678901234567"),&wallet).await?.s;
    assert_ne!(verify_resp,"Verification Done".to_string());
    Ok(())
}


#[tokio::main]
#[test]
async fn verify_test_nonce_not_match() ->  Result<(),Error>
{
    let client = setup_client();
    let wallet = LocalWallet::new(&mut thread_rng());
    let nonce_resp  = nonce_inner(&client).await?.s;
    assert_eq!(nonce_resp.len(),17);

    let verify_resp = verify_inner(&client, &format!("12345678901234567"), &wallet).await?.s;
    assert_eq!(verify_resp,"Custom Rejection: Message nonce does not match".to_string());
    Ok(())
}



async fn verify_inner(client:&Client, nonce_resp:&String, wallet: &LocalWallet) ->  Result<StringStruct, Error> 
{
    
   
    let a = hex::encode(wallet.address());
    let address_hex = eip55::checksum(&a);
    
    let message_mock = format!("localhost:8080 wants you to sign in with your Ethereum account:\n{}\n\nSign in with Ethereum to the app.\n\nURI: http://localhost:8080\nVersion: 1\nChain ID: 1\nNonce: {}\nIssued At: 2023-08-10T10:52:48.787Z", address_hex, nonce_resp);   
    let signature = wallet.sign_message(message_mock.as_str()).await.unwrap();

    let mut map = HashMap::new();
    map.insert("message", message_mock.to_string());
    map.insert("signature", signature.to_string());

    let response = client.post("http://127.0.0.1:3000/verify").json(&map).send().await.unwrap();
    let verify_resp = response.text().await.unwrap();

   
    //println!("{:?}", verify_resp);


    let out: StringStruct = StringStruct{s: verify_resp};
    Ok(out)


    
}






#[tokio::main]
#[test]
async fn personal_info_no_auth_test() ->  Result<(),Error>
{
    let client = setup_client();
    let personal_info = personal_info_inner(&client).await?;
    println!("{}",personal_info.s);
    assert_eq!("You are not authenticated".to_string(),personal_info.s );

    Ok(())
   
}


async fn personal_info_inner(client: &Client) -> Result<StringStruct, Error> {
    let response = client.get("http://127.0.0.1:3000/personal_information").send().await?.text().await?;
    let out  = StringStruct{s:response};
    Ok(out)
}



#[tokio::main]
#[test]
async fn full_integration_test() ->  Result<(),Error>
{
    let client = setup_client();
    let wallet = LocalWallet::new(&mut thread_rng());
    let wallet_address_hex = eip55::checksum(hex::encode(wallet.address()).as_str());
    

    let nonce_resp  = nonce_inner(&client).await?.s;
    assert_eq!(nonce_resp.len(),17);
    let verify_resp = verify_inner(&client, &nonce_resp, &wallet).await?.s;
    assert_eq!(verify_resp,"Verification Done".to_string());
   


    let personal_info = personal_info_inner(&client).await?;
    println!("{}",personal_info.s);
    assert_eq!(format!("You are authenticated and your address is: {wallet_address_hex}").to_string(),   personal_info.s );

    Ok(())
   
}






