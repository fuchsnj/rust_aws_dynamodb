use aws_core::AWSError;

#[derive(Debug)]
pub enum DynamoDbError{
	Core(AWSError)
}


impl From<AWSError> for DynamoDbError {
	fn from(err: AWSError) -> DynamoDbError {
		DynamoDbError::Core(err)
	}
}
