#![feature(plugin)]
#![plugin(json_macros)]

extern crate hyper;
extern crate openssl;
extern crate rustc_serialize;
extern crate rustc_serialize as serialize;
extern crate regex;
extern crate time;
extern crate url;
extern crate xml;
extern crate aws_core;

mod table;
mod types;
mod condition;
mod error;

use aws_core::{AWSError, AWSResult, Credentials, SignedRequest, Region};
use table::Table;
use std::io::Read;
use rustc_serialize::json::{ToJson, Json};
use std::collections::{HashSet, HashMap, BTreeMap};
use std::sync::{Arc, Mutex};
use condition::Condition;
use error::DynamoDbError;

type DynamoDbResult<T> = Result<T, DynamoDbError>;




#[test]
fn it_works() {
	let mut dynamo = DynamoDb::new();
	dynamo.set_credentials("AKIAJPJRPMWISRZIBQYA", "y7tSfQa6bQcFH0QRgK3U2VKGfy5qoru+eb0QFy91");
	//dynamo.list_tables();
	//for _ in 0..200{
	let users_table = dynamo.get_table("users");
	//let item = users_table.get_item(("username", "test")).execute().unwrap();
	//println!("item: {}", item.to_json().unwrap().pretty());
	users_table.put_item(json!({
		"username": "nathan",
		"rand_data": {
			"favorite_banana": "Jana"
		}
	}))
	.condition(Condition::attribute_not_exists("username"))
	.execute().unwrap();
	//}
	panic!("end of tests");
}


fn send_req(req: &mut SignedRequest, creds: &Credentials){
	
}

struct DynamoDbData{
	credentials: Option<Credentials>
}

#[derive(Clone)]
pub struct DynamoDb{
	shared_data: Arc<Mutex<DynamoDbData>>
}


impl DynamoDb{
	fn new() -> DynamoDb{
		DynamoDb{
			shared_data: Arc::new(Mutex::new(
				DynamoDbData{
					credentials: None
				}
			))
		}
	}
	fn get_credentials(&self) -> AWSResult<Credentials>{
		match self.shared_data.lock().unwrap().credentials{
			Some(ref creds) => Ok(creds.clone()),
			None => Err(AWSError::NoCredentials)
		}	
	}
	fn set_credentials(&mut self, id: &str, secret: &str){
		self.shared_data.lock().unwrap().credentials = Some(Credentials{
			id: id.to_owned(),
			secret: secret.to_owned(),
			token: None
		});
	}
	fn get_table(&self, name: &str) -> Table{
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

