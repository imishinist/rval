use anyhow::{Error, Result};

use crate::data::{Response, Spec};
use std::fmt::Display;

fn assert<T, I>(expected: T, got: T, msg: I) -> Result<()>
where
    T: Display + PartialEq,
    I: Into<String>,
{
    if expected != got {
        let context = format!("expected: {}, but got: {}", expected, got);
        return Err(Error::msg(msg.into()).context(context));
    }
    Ok(())
}

pub fn validate(spec: &Spec, response: Response) -> Result<()> {
    if let Some(status) = spec.status {
        assert(status, response.status(), "status invalid")?;
    }
    Ok(())
}
