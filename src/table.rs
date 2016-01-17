use DynamoDb;
use types::{ToPrimaryKey, PrimaryKey, Item, ToItem, FromItem};
use aws_core::{Region, SignedRequest, StatusCode};
use std::io::Read;
use rustc_serialize::json::{Json, ToJson};
use rustc_serialize::json;
use std::fmt::Debug;
use std::collections::BTreeMap;
use condition::Condition;
use DynamoDbResult;
use error::{DynamoDbError, AWSApiError};


#[derive(Clone)]
pub struct Table{
	db: DynamoDb,
	name: String
}
impl Table{
	pub fn new(db: &DynamoDb, name: &str) -> Table{
		Table{
			db: db.clone(),
			name: name.to_owned()
		}
	}
	pub fn get_item<K>(&self, primary_key: K) -> GetItemRequest
	where K: ToPrimaryKey{
		GetItemRequest::new(self, primary_key.to_primary_key())
	}
	pub fn put_item<'a, T>(&self, item: &'a T) -> PutItemRequest<'a, T>
	where T: ToItem{
		PutItemRequest::new(self, item)
	}
	pub fn get_name(&self) -> String{
		self.name.clone()
	}
	pub fn get_db(&self) -> DynamoDb{
		self.db.clone()
	}
}
pub struct PutItemRequest<'a, T: 'a>
where T: ToItem{
	table: Table,
	item: &'a T,
	condition: Option<Condition>
}
impl<'a, T> PutItemRequest<'a, T> where T: ToItem{
	fn new(table: &Table, item: &'a T) -> PutItemRequest<'a, T>{
		PutItemRequest{
			table: table.clone(),
			item: item,
			condition: None
		}
	}
	pub fn execute(&self) -> DynamoDbResult<()>{
		println!("executing item put");
		let item:Item = try!(self.item.to_item());
		println!("item to put: {:?}", item);
		
		let mut req = SignedRequest::new("POST", "dynamodb", Region::UsEast1, "/");
		let mut map = BTreeMap::new();
		map.insert("TableName".to_string(), self.table.get_name().to_json());
		map.insert("Item".to_string(), try!(item.to_typed_map()));
		if let Some(ref condition) = self.condition{
			map.insert("ConditionExpression".to_string(), condition.to_raw_string().to_json());
		}
		let json = Json::Object(map);
		println!("puut request: {}", json);
		println!("body: {}", json);
		req.set_payload(json.to_string().as_bytes());
		req.add_header("X-Amz-Target", "DynamoDB_20120810.PutItem");
		let creds = try!(self.table.get_db().get_credentials());
		let mut res = req.sign_and_execute(&creds);
		let mut res_body = String::new();
		res.read_to_string(&mut res_body).unwrap();
		println!("res: {}", res_body);
		match res.status{
			StatusCode::Ok => Ok(()),
			StatusCode::BadRequest => {
				println!("bad request received!");
				let err: AWSApiError = try!(json::decode(&res_body));
				match err.__type.as_ref(){
					"com.amazonaws.dynamodb.v20120810#ConditionalCheckFailedException" => Err(DynamoDbError::ConditionFailed),
					err @ _ => {
						panic!(format!("unknown error type: {}", err))
					}
				}
			},
			code @ _ => panic!("unknown return status code {:?}", code)
		}
	}
	pub fn condition(mut self, cond: Condition) -> PutItemRequest<'a, T>{
		self.condition = Some(cond);
		self
	}
}

pub struct GetItemRequest{
	table: Table,
	primary_key: PrimaryKey
}
impl GetItemRequest{
	fn new(table: &Table, primary_key: PrimaryKey) -> GetItemRequest{
		GetItemRequest{
			table: table.clone(),
			primary_key: primary_key
		}
	}
	pub fn execute<T>(self) -> DynamoDbResult<Option<T>>
	where T: FromItem{
		let mut req = SignedRequest::new("POST", "dynamodb", Region::UsEast1, "/");
		let json = json!({
			"TableName" => (self.table.get_name()),
			"Key" => (self.primary_key.to_primary_key())
		}).to_string();
		println!("get request: {}", json);
		req.set_payload(json.as_bytes());
		req.add_header("X-Amz-Target", "DynamoDB_20120810.GetItem");
		let creds = try!(self.table.get_db().get_credentials());
		let mut res = req.sign_and_execute(&creds);
		
		let mut msg = String::new();
		res.read_to_string(&mut msg).unwrap();
		println!("get row res: {}", msg);
		let json = Json::from_str(&msg).unwrap();
		let json_item = match json.find("Item"){
			Some(item) => item,
			None => return Ok(None)
		};
		let item = try!(Item::from_typed_map(json_item));
		Ok(Some(try!(T::from_item(item))))
	}
}