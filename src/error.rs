use aws_core::AWSError;
use rustc_serialize::json::EncoderError;

#[derive(Debug)]
pub enum DynamoDbError{
	Core(AWSError),
	EncodingError
}


impl From<AWSError> for DynamoDbError {
	fn from(err: AWSError) -> DynamoDbError {
		DynamoDbError::Core(err)
	}
}

impl From<EncoderError> for DynamoDbError{
	fn from(err: EncoderError) -> DynamoDbError{
		DynamoDbError::EncodingError
	}
}