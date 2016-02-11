use aws_core::AWSError;
use serde_json;

#[derive(Debug)]
pub enum DynamoDbError{
	Core(AWSError),
	DeserializeError(serde_json::error::Error),
	ConditionFailed
}

impl From<AWSError> for DynamoDbError {
	fn from(err: AWSError) -> DynamoDbError {
		DynamoDbError::Core(err)
	}
}

impl From<serde_json::error::Error> for DynamoDbError{
	fn from(err: serde_json::error::Error) -> DynamoDbError{
		DynamoDbError::DeserializeError(err)
	}
}

#[derive(Deserialize, Debug)]
pub struct AWSApiError{
	pub __type: String
}