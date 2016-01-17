use aws_core::AWSError;
use rustc_serialize::json::{EncoderError, ParserError, DecoderError};

#[derive(Debug)]
pub enum DynamoDbError{
	Core(AWSError),
	EncodingError(EncoderError),
	DecodingError(DecoderError),
	ParsingError(ParserError),
	ConditionFailed
}


impl From<AWSError> for DynamoDbError {
	fn from(err: AWSError) -> DynamoDbError {
		DynamoDbError::Core(err)
	}
}

impl From<EncoderError> for DynamoDbError{
	fn from(err: EncoderError) -> DynamoDbError{
		DynamoDbError::EncodingError(err)
	}
}

impl From<ParserError> for DynamoDbError{
	fn from(err: ParserError) -> DynamoDbError{
		DynamoDbError::ParsingError(err)
	}
}

impl From<DecoderError> for DynamoDbError{
	fn from(err: DecoderError) -> DynamoDbError{
		DynamoDbError::DecodingError(err)
	}
}

#[derive(RustcDecodable, Debug)]
pub struct AWSApiError{
	pub __type: String
}