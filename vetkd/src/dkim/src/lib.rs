use email::EmailMessage;
use ic_cdk::api::call::RejectionCode;
use regex::Regex;
use resolver::Resolver;
use viadkim::{signature, VerificationResult, VerificationStatus, Verifier};

use std::cell::RefCell;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH, Duration};
// use std::{error::Error, fs, io};
use serde::{Serialize, Deserialize};
use candid::{CandidType,};


pub mod types;


const VETKD_SYSTEM_API_CANISTER_ID: &str = "bw4dl-smaaa-aaaaa-qaacq-cai";


#[derive(Serialize, CandidType)]
pub enum DkimStatus {
    Success,
    Failure
}

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
    Comment: Option<String>,
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



#[derive(CandidType)]
struct DkimVerification {
    status: DkimStatus,
    to: String,
    subject: String
}
use ic_cdk::api::management_canister::http_request::http_request;
use ic_cdk::api::management_canister::http_request::{HttpResponse, TransformArgs, HttpHeader, CanisterHttpRequestArgument, HttpMethod, TransformContext, TransformFunc, self};
pub mod email;
pub mod resolver;



thread_local! {
    static EMAILS:RefCell<Vec<String>> = RefCell::new(vec![]);
    static EMAIL_OTPS :RefCell<HashMap<u16,String>> = RefCell::new(HashMap::new());
    static EMAIL_SECRETS:RefCell<HashMap<String,String>> = RefCell::new(HashMap::new());
} 
async fn verify_email(raw_email: String) -> Result<DkimVerification,String> {
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

    let time_string = &ic_cdk::api::time().to_string()[..10];

    let time :u64= time_string.parse().unwrap(); 

    let config = viadkim::Config {
        fixed_system_time: Some(UNIX_EPOCH + Duration::from_secs(time)),
        allow_expired: true, 
        ..Default::default()

    };
    
    let mut verifier = match Verifier::verify_header(&resolver, &headers, &config).await {
        Some(verifier) => verifier,
        None => {
            eprintln!("No signatures in input message");
            return Err("No signatures in input message".to_string());
        }
    };

    let _ = verifier.process_body_chunk(body.as_bytes());

    let sigs = verifier.finish();
    // sigs
    let selected_headers = headers.into_iter().filter(|(key,_)| key.to_string() == "To" || key.to_string() == "Subject").collect::<Vec<_>>();
    // let subject = headers.into_iter().filter(|(key,_)| key.to_string() == "Subject").collect::<Vec<_>>();
    // ic_cdk::println!("To header: {:?}", to_header);
    // sigs.
    // for (i, sig) in sigs.into_iter().enumerate() {
    //     ic_cdk::println!("\n");g
    //     ic_cdk::println!("SIGNATURE {}", i + 1);
    //     ic_cdk::println!("{sig:#?}");
    // }
    let signature = sigs.into_iter().next().unwrap();
    ic_cdk::println!("To header: {:?}",selected_headers[1].1);
    ic_cdk::println!("Subject header: {:?}", selected_headers[0].1);

    let verifaction_result:  Result<DkimVerification, _> = match signature.status {
        VerificationStatus::Success => {
            ic_cdk::println!("Signature verified successfully");
           
            Ok::<DkimVerification,String>(
                DkimVerification {
                    status: DkimStatus::Success,
                    subject: format!("{:?}",selected_headers[0].1),
                    to: format!("{:?}",selected_headers[1].1)
                }
            )
        }
        VerificationStatus::Failure(_) => {
            ic_cdk::println!("Signature verification failed");
            Ok(
                DkimVerification {
                    status: DkimStatus::Failure,
                    subject: format!("{:?}",selected_headers[0].1),
                    to: format!("{:?}",selected_headers[1].1)
                }
            )
        }
    };
    let output: DkimVerification = verifaction_result.unwrap();

    Ok(output)
    
}

#[ic_cdk::update]
async fn register_email(email: String) -> Result<u16, String> {
    // let email_hash = hash_string(email.as_str());
    let email_hashmap = EMAILS.with(|cell|{
       cell.borrow().clone()
    });
    ic_cdk::println!("{}", email);
    if email_hashmap.contains(&email){
        return Err("Email already registered".to_string());
    }
    let num = fastrand::u16(101..999);
    EMAIL_OTPS.with(|cell|{
        cell.borrow_mut().insert(num.into(),email.clone());
    });
    Ok((num))
}

#[ic_cdk::update]
async fn get_otp(email: String) -> Result<u16, String> {
    let email_hashmap = EMAILS.with(|cell|{
       cell.borrow().clone()
    });
    if email_hashmap.contains(&email){
        let num: u16 = fastrand::u16(101..999);
        EMAIL_OTPS.with(|cell|{
            cell.borrow_mut().insert(num,email.clone());
        });
        Ok((num))
    } else {
        Err("Email not registered".to_string())
    }
}


#[ic_cdk::update]
async fn finalize_secret_with_email(email_bytes: String,secret_bytes: String) -> Result<(),String> {
 ic_cdk::println!("Email: {}",email_bytes);
 ic_cdk::println!("Secret: {}",secret_bytes);
 
 let verification_details = verify_email(email_bytes).await.unwrap();
//  ic_cdk::println!("Emails: {:?}", verification_details.subject.as_bytes());
 ic_cdk::println!("Numberrrr:{}", verification_details.subject.to_ascii_lowercase().replace(" ", ""));
 let result: Result<(), String> = match verification_details.status {
     DkimStatus::Success => {
        let otp_from_email = verification_details.subject.to_string().to_ascii_lowercase().replace(" ", "").replace("\"","").parse::<u16>().unwrap();
        let otp_hashmap = EMAIL_OTPS.with(|cell|{
            cell.borrow().clone()
        });
        
        let email_from_otp_option = otp_hashmap.get(&otp_from_email);
        match email_from_otp_option {
            Some(email_from_otp) => {
                if verification_details.to.contains(email_from_otp) {
                    EMAIL_SECRETS.with(|cell|{
                        cell.borrow_mut().insert(email_from_otp.to_string(),secret_bytes.to_string());
                    });
                    EMAILS.with(|cell|{
                        cell.borrow_mut().push(email_from_otp.clone());
                    });
                    return Ok(());
                } else {
                    return Err("Email verification failed".to_string());
                }
            },
            None =>  return Err("Email verification failed".to_string()),
        }
        Ok(())
     },
     DkimStatus::Failure => {
         Err("Email verification failed".to_string())
     }
 };
 result
}

#[ic_cdk::update]
async fn retrieve_secret(email_bytes: String) -> Result<String,String> {
 let verification_details = verify_email(email_bytes).await.unwrap();
 let result: Result<String, String> = match verification_details.status {
     DkimStatus::Success => {
        let otp_from_email = verification_details.subject.to_ascii_lowercase().replace(" ", "").replace("\"","").parse::<u16>().unwrap();
        let otp_hashmap = EMAIL_OTPS.with(|cell|{
            cell.borrow().clone()
        });
        
        let email_from_otp_option = otp_hashmap.get(&otp_from_email);
        match email_from_otp_option {
            Some(email_from_otp) => {
                if verification_details.to.contains(email_from_otp) {
                    let secret_hashmap = EMAIL_SECRETS.with(|cell|{
                        cell.borrow().clone()
                    });
                    let secret = secret_hashmap.get(email_from_otp).unwrap();
                    return Ok(secret.to_string());
                } else {
                    return Err("Email verification failed".to_string());
                }
            },
            None =>  return Err("Email verification failed".to_string()),
        }
     },
     DkimStatus::Failure => {
         Err("Email verification failed".to_string())
     }
 };
 result
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
        Err(err) => {
            ic_cdk::println!("print ln {:?}",err.0);
            ic_cdk::println!("printllnnasdfds {:?}",err.1);
            "".to_string()
        }, 
    }


}


ic_cdk::export_candid!();