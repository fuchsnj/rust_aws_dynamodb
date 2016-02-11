#![feature(custom_derive, custom_attribute, plugin)]
#![plugin(serde_macros)]
#![plugin(json_macros)]

extern crate hyper;
extern crate openssl;
extern crate regex;
extern crate time;
extern crate url;
extern crate xml;
extern crate aws_core;
extern crate serde;
extern crate serde_json;

pub mod table;
pub mod types;
mod condition;
mod error;

use aws_core::{AWSError, AWSResult, Credentials};
use std::sync::{Arc, Mutex};

pub use error::DynamoDbError;
pub use condition::Condition;
pub use table::Table;

pub type DynamoDbResult<T> = Result<T, DynamoDbError>;

struct DynamoDbData{
	credentials: Option<Credentials>
}

#[derive(Clone)]
pub struct DynamoDb{
	shared_data: Arc<Mutex<DynamoDbData>>
}


impl DynamoDb{
	pub fn new() -> DynamoDb{
		DynamoDb{
			shared_data: Arc::new(Mutex::new(
				DynamoDbData{
					credentials: None
				}
			))
		}
	}
	pub fn get_credentials(&self) -> AWSResult<Credentials>{
		match self.shared_data.lock().unwrap().credentials{
			Some(ref creds) => Ok(creds.clone()),
			None => Err(AWSError::NoCredentials)
		}	
	}
	pub fn set_credentials(&mut self, id: &str, secret: &str){
		self.shared_data.lock().unwrap().credentials = Some(Credentials{
			id: id.to_owned(),
			secret: secret.to_owned(),
			token: None
		});
	}
	pub fn get_table(&self, name: &str) -> Table{
		Table::new(self, name)
	}
	/*
	fn list_tables(&self){
		let mut req = SignedRequest::new("POST", "dynamodb", Region::UsEast1, "/");
		let json = json!({
			
		}).to_string();
		req.set_payload(json.as_bytes());
		req.add_header("X-Amz-Target", "DynamoDB_20120810.ListTables");
		if let Some(ref creds) = self.credentials{
			let mut res = req.sign_and_execute(creds);
			println!("response: {:#?}", res);
			let mut msg = String::new();
			res.read_to_string(&mut msg).unwrap();
			println!("msg: {:?}", msg);
		}else{
			println!("no credentials!");
		}
	}*/
}

