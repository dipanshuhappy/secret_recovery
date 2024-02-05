use email::EmailMessage;
use regex::Regex;
use resolver::Resolver;
use viadkim::{Verifier};

use std::time::{SystemTime, UNIX_EPOCH, Duration};
// use std::{error::Error, fs, io};
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



// thread_local! {
//     static EMAIL_HASHES :RefCell<Vec<String>> = RefCell::new(vec![]);
//     static EMAIL_OTPS :RefCell<HashMap<String,u16>> = RefCell::new(HashMap::new());
// }
async fn verify_email(raw_email: String) -> Result<(),String> {
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
    let email_message: Result<EmailMessage, String> = email::parse_message(&raw_email).map(|m| m)
        .map_err(|e| format!("Error parsing message: {}", e));

    let EmailMessage { body_string: body, header_fields: headers, header_string} = email_message.unwrap();
    // for (key,value) in headers.clone().into_iter(){
    //    if key.to_string() == "To" {
    //        let text = format!("{:?}",value);
    //        let email_regex = Regex::new(r"[\w\.-]+@[\w\.-]+\.\w+").unwrap();
    //        match email_regex.find(&text) {
    //         Some(matched) => println!("{}", matched.as_str()),
    //         None => println!("No email found"),
    //        }
    //    }
    // }
    
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

    let to_header = headers.into_iter().filter(|(key,_)| key.to_string() == "To").collect::<Vec<_>>();

    // for (i, sig) in sigs.into_iter().enumerate() {
    //     ic_cdk::println!("\n");
    //     ic_cdk::println!("SIGNATURE {}", i + 1);
    //     ic_cdk::println!("{sig:#?}");
    // }

    Ok(())


    
}
// fn hash_string (email: &str) -> String{
//     use sha3::{Digest,Sha3_256};
//     let mut hasher = Sha3_256::new();
//     hasher.update(email);
//     let result = hasher.finalize();
//     hex::encode(result)
// }

// #[ic_cdk::update]
// async fn register_email(email: String,) -> Result<(u8), String> {
//     let email_hash = hash_string(email.as_str());
//     let email_hashmap = EMAIL_HASHES.with(|cell|{
//        cell.borrow().clone()
//     });
//     ic_cdk::println!("{}", email_hash);
//     if email_hashmap.contains(&email_hash){
//         return Err("Email already registered".to_string());
//     }
//     let num = fastrand::u8(1..100);
//     EMAIL_OTPS.with(|cell|{
//         cell.borrow_mut().insert(email_hash.clone(),num as u16);
//     });
//     EMAIL_HASHES.with(|cell|{
//         cell.borrow_mut().push(email_hash.clone());
//     });
//     Ok((num))
// }

#[ic_cdk::update]
async fn finalize_secret_with_email(email_bytes: String) -> Result<(),String> {
 verify_email(email_bytes).await;
 Ok(())
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


ic_cdk::export_candid!();