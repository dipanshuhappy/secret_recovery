use email::EmailMessage;
use resolver::Resolver;
use viadkim::{Config, Verifier, VerificationResult};

use std::time::{SystemTime, UNIX_EPOCH, Duration};
use std::{error::Error, fs, io};
use serde::{Serialize, Deserialize};



#[derive(Serialize, Deserialize)]
struct DnsQueryResponse {
    Status: i32,
    TC: bool,
    RD: bool,
    RA: bool,
    AD: bool,
    CD: bool,
    Question: Vec<DnsQuestion>,
    Answer: Vec<DnsAnswer>,
    Comment: String,
}

#[derive(Serialize, Deserialize)]
struct DnsQuestion {
    name: String,
    #[serde(rename = "type")]
    type_: i32,
}

#[derive(Serialize, Deserialize)]
struct DnsAnswer {
    name: String,
    #[serde(rename = "type")]
    type_: i32,
    TTL: i32,
    data: String,
}

use ic_cdk::api::management_canister::http_request::http_request;
use ic_cdk::api::management_canister::http_request::{HttpResponse, TransformArgs, HttpHeader, CanisterHttpRequestArgument, HttpMethod, TransformContext, TransformFunc, self};
pub mod email;
pub mod resolver;

#[ic_cdk::update]
async fn run_command(msg:String) -> Result<(), String> {
    ic_cdk::print("hiii");
    // let msg = match &config.in_file {
    //     Some(f) => fs::read_to_string(f)?,
    //     None => io::read_to_string(io::stdin())?,
    // };
    
    // let Ok(EmailMessage { body_string: body, header_fields: headers, .. }) =
    //     email::parse_message(&msg).map(|m| m)
    //     .map_err(|e| format!("Error parsing message: {}", e));

    let email_message = email::parse_message(&msg).map(|m| m)
        .map_err(|e| format!("Error parsing message: {}", e));
    let EmailMessage { body_string: body, header_fields: headers, .. } = email_message.unwrap();

    let resolver = Resolver::new(|name|{
        Box::pin(async move {
            ic_cdk::println!("name: {}",name);
            let dns_string = get_dkim(name.to_string()).await;
            ic_cdk::println!("dns_string: {}",dns_string);
            let dns_response: DnsQueryResponse = serde_json::from_str(&dns_string).unwrap();
            let final_response: &String = &dns_response.Answer[0].data;
            return Ok(vec![Ok(format!("{final_response}").into())])
        })
    });

    let time_string = &ic_cdk::api::time().to_string()[..10];

    let time :u64= time_string.parse().unwrap(); 

    let config = viadkim::Config {
        fixed_system_time: Some(UNIX_EPOCH + Duration::from_secs(time)),
        ..Default::default()

    };

    let mut verifier = match Verifier::verify_header(&resolver, &headers, &config).await {
        Some(verifier) => verifier,
        None => {
            eprintln!("No signatures in input message");
            return Ok(());
        }
    };

    let _ = verifier.process_body_chunk(body.as_bytes());

    let sigs = verifier.finish();

    for (i, sig) in sigs.into_iter().enumerate() {
        ic_cdk::println!("\n");
        ic_cdk::println!("SIGNATURE {}", i + 1);
        ic_cdk::println!("{sig:#?}");
    }

    Ok(())
}

#[ic_cdk::query]
fn time() -> u64 {
    ic_cdk::api::time()
}
#[ic_cdk::query]
fn transform(response: TransformArgs) -> HttpResponse {
    let mut t = response.response;
    t.headers = vec![];
    t 
}

#[ic_cdk::update]
async fn get_dkim(name:String)->String {
    let url  = format!("https://dns.google.com/resolve?name={}&type=TXT",name);
    let request_header = vec![
        HttpHeader{
            name:"Accept".to_string(),
            value:"*/*".to_string()
        }
    ];
    let request = CanisterHttpRequestArgument{
        url:url.to_string(),
        method:HttpMethod::GET,
        body:None,
        max_response_bytes:None,
        transform:Some(
            TransformContext{
                function:TransformFunc(candid::Func { principal: ic_cdk::api::id(), method: "transform".to_string() }),
                context:vec![]
            }
        ),
        headers:request_header
    };

    match http_request(request,230_949_972_000).await {
        Ok((response,))=>{
            let str_body=String::from_utf8(response.body).expect("Transformed response is not UTF-8 encoded.");
            str_body
        },
    Err(_) => todo!(), 
    }


}
#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}


// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[derive(CandidType)]
// struct Argument {
//   amount: Option<Nat>,
// }

// #[derive(CandidType, Deserialize)]
// struct CreateCanisterResult {
//   canister_id: Principal,
// }
// use ic_agent::{Agent, export::Principal};
// use candid::{Encode, Decode, CandidType, Nat};
// use serde::Deserialize;


// async fn create_a_canister() -> Result<Principal, Box<dyn std::error::Error>> {
//     let agent = Agent::builder()
//     // Use the URL of the Internet Computer or a local replica
//     .with_url("http://127.0.0.1:4943")
    
//     .with_identity(create_identity())
//     // todo: how to simulate : dfx use btwl1 ? and the account have passwd. when i exec dfx cmd . it requires my passwd in bash
//     .build()?;

//     agent.fetch_root_key().await?;
//     let backend_canister_id = Principal::from_text("bd3sg-teaaa-aaaaa-qaaba-cai")?;

//     let response = agent.update(&backend_canister_id,"run_command").with_arg("hi").call_and_wait().await?;


//   }
//     #[test]

//     pub fn testing(){
//         create_a_canister()





//     }


    
       
// }

ic_cdk::export_candid!();