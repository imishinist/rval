use anyhow::{Error, Result};

use crate::data::{Response, Spec};

pub fn validate(spec: &Spec, response: Response) -> Result<()> {
    if let Some(status) = spec.status {
        if status != response.status() {
            let context = format!("expected: {}, but got: {}", status, response.status());
            return Err(Error::msg("status invalid").context(context));
        }
    }
    Ok(())
}
