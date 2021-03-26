use anyhow::{Error, Result};

use crate::data::{Response, Spec};
use std::fmt::{Debug, Display};

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

fn contains<T, I>(expected: &T, got: &[T], msg: I) -> Result<()>
where
    T: Debug + PartialEq,
    I: Into<String>,
{
    for h in got {
        if expected == h {
            return Ok(());
        }
    }
    let context = format!("expected: {:?}, but don't contains", expected);
    Err(Error::msg(msg.into()).context(context))
}

pub fn validate(spec: &Spec, response: Response) -> Result<()> {
    if let Some(status) = spec.status() {
        assert(status, response.status(), "status invalid")?;
    }
    if let Some(header) = spec.header() {
        contains(header, response.headers(), "header don't contains")?;
    }
    Ok(())
}
