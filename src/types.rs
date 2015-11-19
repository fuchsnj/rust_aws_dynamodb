use std::collections::{HashSet, HashMap, BTreeMap};
use rustc_serialize::json::{ToJson, Json};
use rustc_serialize::json;
use rustc_serialize::{Decodable, Encodable};
use DynamoDbResult;
use aws_core::AWSError;
use error::DynamoDbError;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Number{
	value: String
}

pub struct KeyValuePair{
	key: String,
	value: Item
}

#[derive(Clone)]
pub struct PrimaryKeyValuePair{
	key: String,
	value: PrimaryKeyValue
}
impl PrimaryKeyValuePair{
	fn new<V>(key: &str, value: V) -> PrimaryKeyValuePair
	where V: ToPrimaryKeyValue{
		PrimaryKeyValuePair{
			key: key.to_owned(),
			value: value.to_primary_key_value()
		}
	}
}

pub trait ToPrimaryKeyValuePair{
	fn to_primary_key_value_pair(&self) -> PrimaryKeyValuePair;
}
impl ToPrimaryKeyValuePair for PrimaryKeyValuePair{
	fn to_primary_key_value_pair(&self) -> PrimaryKeyValuePair{
		self.clone()
	}
}
impl<'a, V> ToPrimaryKeyValuePair for (&'a str, V)
where V: ToPrimaryKeyValue{
	fn to_primary_key_value_pair(&self) -> PrimaryKeyValuePair{
		PrimaryKeyValuePair{
			key: self.0.to_owned(),
			value: self.1.to_primary_key_value()
		}
	}
}

#[derive(Clone)]
pub struct PrimaryKey{
	hash: PrimaryKeyValuePair,
	range: Option<PrimaryKeyValuePair>
}
impl PrimaryKey{
	fn hash<K>(key_value: K) -> PrimaryKey
	where K: ToPrimaryKeyValuePair{
		PrimaryKey{
			hash: key_value.to_primary_key_value_pair(),
			range: None
		}
	}
}
impl ToJson for PrimaryKey{
	fn to_json(&self) -> Json{
		let mut map = BTreeMap::new();
		map.insert(self.hash.key.clone(), self.hash.value.to_json());
		if let Some(ref range) = self.range{
			map.insert(range.key.clone(), range.value.to_json());
		} 
		Json::Object(map)
	}
}
impl ToJson for PrimaryKeyValue{
	fn to_json(&self) -> Json{
		match *self{
			PrimaryKeyValue::String(ref val) => json!({
				"S" => (val)
			}),
			PrimaryKeyValue::Number(ref num) => json!({
				"N" => (num.value)
			}),
			PrimaryKeyValue::Binary(ref val) => panic!("binary type not yet supported")
		}
	}
}

pub trait ToPrimaryKey{
	fn to_primary_key(self) -> PrimaryKey;
}
impl ToPrimaryKey for PrimaryKey{
	fn to_primary_key(self) -> PrimaryKey{
		self.clone()
	}
}
impl<'a, V> ToPrimaryKey for (&'a str, V)
where V: ToPrimaryKeyValue{
	fn to_primary_key(self) -> PrimaryKey{
		PrimaryKey{
			hash: self.to_primary_key_value_pair(),
			range: None
		}
	}
}
impl<'a, V1, V2> ToPrimaryKey for (&'a str, V1, &'a str, V2)
where V1: ToPrimaryKeyValue, V2: ToPrimaryKeyValue{
	fn to_primary_key(self) -> PrimaryKey{
		PrimaryKey{
			hash: (self.0, self.1).to_primary_key_value_pair(),
			range: Some((self.2, self.3).to_primary_key_value_pair())
		}
	}
}


#[derive(Clone)]
enum PrimaryKeyValue{
	String(String),
	Number(Number),
	Binary(Vec<u8>)
}

pub trait ToItem{
	fn to_item(&self) -> DynamoDbResult<Item>;
}
/*impl ToItem for Json{
	fn to_item(&self) -> AWSResult<Item>{
		Item::from_json(self)
	}
}*/
impl<T> ToItem for T where T: Encodable{
	fn to_item(&self) -> DynamoDbResult<Item>{
		let encoded = try!(json::encode(self));
		println!("to_item: {}", encoded);
		let json = try!(Json::from_str(&encoded));
		Item::from_json(&json)
	}
}

pub trait FromItem{
	fn from_item(item: Item) -> DynamoDbResult<Self>;
}
impl<T> FromItem for T where T: Decodable{
	fn from_item(item: Item) -> DynamoDbResult<T>{
		let json = try!(item.to_json());
		let decoded:T = try!(json::decode(&json.to_string()));
		Ok(decoded)
	}
}

#[derive(Debug)]
pub enum Item{
	Number(Number),//N
	String(String),//S
	Binary(Vec<u8>),//B
	Null,//NULL
	StringSet(HashSet<String>),//SS
	NumberSet(HashSet<Number>),//NS
	BinarySet(HashSet<Vec<u8>>),//BS
	List(Vec<Item>),//L
	Map(HashMap<String, Item>)//M
}
impl Item{
	fn from_typed_json(json: &Json) -> DynamoDbResult<Item>{
		if let Some(val) = json.find("N"){
			panic!("N");
		}else if let Some(val) = json.find("S"){
			match val.as_string(){
				Some(value) => Ok(Item::String(value.to_owned())),
				None => try!(Err(AWSError::protocol_error("type S must be a string")))
			}
		}else if let Some(val) = json.find("M"){
			Item::from_typed_map(val)
		}else{
			panic!("UNKNOWN: from_typed_json");
		}
	}
	pub fn to_typed_json(&self) -> Json{
		match *self{
			Item::Map(ref data) => {
				let mut map = BTreeMap::new();
				for (key, value) in data.iter(){
					map.insert(key.to_owned(), value.to_typed_json());
				}
				json!({
					"M" => (Json::Object(map))
				})
			},
			Item::String(ref value) => {
				json!({
					"S" => (Json::String(value.to_owned()))
				})
			},
			Item::List(ref array) => {
				let mut output = vec!();
				for a in array{
					output.push(a.to_typed_json());
				}
				json!({
					"L" => (Json::Array(output))
				})
			},
			Item::Null => json!({
				"NULL" => ("true")
			}),
			ref variant @ _ => panic!("unknown variant in Item::to_typed_json: {:?}", variant)
		}
	}
	pub fn to_typed_map(&self) -> DynamoDbResult<Json>{
		match *self{
			Item::Map(ref data) => {
				let mut map = BTreeMap::new();
				for (key, value) in data.iter(){
					map.insert(key.to_owned(), value.to_typed_json());
				}
				Ok(Json::Object(map))
			},
			_ => try!(Err(AWSError::protocol_error("Top level type must be a Map")))
		}
	}
	pub fn from_typed_map(json: &Json) -> DynamoDbResult<Item>{
		let data = match json.as_object(){
			Some(val) => val,
			None => try!(Err(AWSError::protocol_error("invalid MAP data")))
		};
		let mut map = HashMap::new();
		for (key, value) in data{
			map.insert(key.to_owned(), try!(Item::from_typed_json(value)));	
		}
		Ok(Item::Map(map))
	}
	pub fn from_json(json: &Json) -> DynamoDbResult<Item>{
		match *json{
			Json::Object(ref json) => {
				let mut map = HashMap::new();
				for (key, value) in json{
					map.insert(key.to_owned(), try!(value.to_item()));
				}
				Ok(Item::Map(map))
			},
			Json::String(ref value) => {
				Ok(Item::String(value.to_owned()))
			},
			Json::Array(ref array) => {
				let mut output = vec!();
				for a in array{
					output.push(try!(a.to_item()));
				}
				Ok(Item::List(output))
			}
			Json::Null => Ok(Item::Null),
			ref variant @ _ => panic!("unknown variant in Item::from_json: {:?}", variant)
		}
	}
	pub fn to_json(&self) -> DynamoDbResult<Json>{
		match *self{
			Item::Map(ref map) => {
				let mut json = BTreeMap::new();
				for (key, value) in map{
					json.insert(key.to_owned(), try!(value.to_json()));
				}
				Ok(Json::Object(json))
			},
			Item::String(ref value) => Ok(Json::String(value.to_owned())),
			_ => panic!("to_json: unknown type")
		}
	}
}




trait ToPrimaryKeyValue{
	fn to_primary_key_value(&self) -> PrimaryKeyValue;
}
impl ToPrimaryKeyValue for PrimaryKeyValue{
	fn to_primary_key_value(&self) -> PrimaryKeyValue{
		self.clone()
	}
}
impl<'a> ToPrimaryKeyValue for &'a str{
	fn to_primary_key_value(&self) -> PrimaryKeyValue{
		PrimaryKeyValue::String((*self).to_owned())
	}
}


