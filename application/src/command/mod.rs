use std::error::Error;
use std::fmt;

pub mod user;

pub trait Command<REQUEST> {}

#[derive(Debug, Clone)]
struct NotFoundException;

#[derive(Debug, Clone)]
struct OperationNotAuthorizedException;

#[derive(Debug, Clone)]
struct ValidationException;

impl fmt::Display for NotFoundException {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Not Found Exception")
    }
}

impl fmt::Display for OperationNotAuthorizedException {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Operation Not Authorized Exception")
    }
}

impl fmt::Display for ValidationException {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Validation Exception")
    }
}

impl Error for NotFoundException {}

impl Error for OperationNotAuthorizedException {}

impl Error for ValidationException {}

pub trait CommandHandler<REQUEST, RESPONSE>
    where REQUEST: Command<RESPONSE>,
{
    fn handle(&self, command: REQUEST) -> Result<RESPONSE, Box<dyn Error>>;
}