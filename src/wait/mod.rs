//! Wait module collect the wait functions
//! Once a relayer submit a header and wait the time in blocks after the calculated value from wait
//! function, Darwinia network will deem this header is valided and become a best header.
//! There is only linear module at first.
use crate::error::Error;

pub mod linear;

pub trait ConfigValidate {
    fn validate(&self) -> Result<(), Error>;
}
