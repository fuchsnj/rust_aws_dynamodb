use DynamoDb;
use types::{ToPrimaryKey, PrimaryKey, Item, ToItem};
use aws_core::{AWSResult, Region, SignedRequest};
use std::io::Read;
use rustc_serialize::json::{Json, ToJson};
use std::fmt::Debug;
use std::collections::BTreeMap;
use condition::Condition;
use DynamoDbResult;

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
	pub fn put_item<T>(&self, item: T) -> PutItemRequest<T>
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
pub struct PutItemRequest<T>
where T: ToItem{
	table: Table,
	item: T,
	condition: Option<Condition>
}
impl<T> PutItemRequest<T> where T: ToItem{
	fn new(table: &Table, item: T) -> PutItemRequest<T>{
		PutItemRequest{
			table: table.clone(),
			item: item,
			condition: None
		}
	}
	pub fn execute(&self) -> DynamoDbResult<()>{
		let item:Item = try!(self.item.to_item());
		println!("item to put: {:?}", item);
		
		let mut req = SignedRequest::new("POST", "dynamodb", Region::UsEast1, "/");
		/*let mut json = json!({
			"TableName": (self.table.get_name()),
			"Item": (try!(item.to_typed_map()))
		});*/
		let mut map = BTreeMap::new();
		map.insert("TableName".to_string(), self.table.get_name().to_json());
		map.insert("Item".to_string(), try!(item.to_typed_map()));
		if let Some(ref condition) = self.condition{
			map.insert("ConditionExpression".to_string(), condition.to_raw_string().to_json());
		}
		let json = Json::Object(map);
		println!("body: {}", json);
		req.set_payload(json.to_string().as_bytes());
		req.add_header("X-Amz-Target", "DynamoDB_20120810.PutItem");
		let creds = try!(self.table.get_db().get_credentials());
		let mut res = req.sign_and_execute(&creds);
		let mut msg = String::new();
		res.read_to_string(&mut msg).unwrap();
		println!("res: {}", msg);
		//let json = Json::from_str(&msg).unwrap();
		//let item = json.find("Item").unwrap();
		//Item::from_typed_map(item)
		panic!("end of PutItem execute()");
	}
	pub fn condition(mut self, cond: Condition) -> PutItemRequest<T>{
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
	pub fn execute(self) -> AWSResult<Item>{
		let mut req = SignedRequest::new("POST", "dynamodb", Region::UsEast1, "/");
		let json = json!({
			"TableName": (self.table.get_name()),
			"Key": (self.primary_key.to_primary_key())
		}).to_string();
		req.set_payload(json.as_bytes());
		req.add_header("X-Amz-Target", "DynamoDB_20120810.GetItem");
		let creds = try!(self.table.get_db().get_credentials());
		let mut res = req.sign_and_execute(&creds);
		let mut msg = String::new();
		res.read_to_string(&mut msg).unwrap();
		let json = Json::from_str(&msg).unwrap();
		let item = json.find("Item").unwrap();
		Item::from_typed_map(item)
	}
}