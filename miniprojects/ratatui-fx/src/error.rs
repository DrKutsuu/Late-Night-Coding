#[derive(Debug)]
pub struct UnexpectedError {
    message: String
}

impl UnexpectedError {
    pub fn new(message: String) -> UnexpectedError {
        UnexpectedError { message }
    }
}

/*
    This allows us to catch any error using ? and wrap it into our own type for easier use
 */
impl From<std::io::Error> for UnexpectedError {
    fn from(io_error: std::io::Error) -> UnexpectedError {
        UnexpectedError::new(io_error.to_string())
    }
}
