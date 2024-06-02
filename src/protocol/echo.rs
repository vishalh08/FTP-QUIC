use color_eyre::eyre::{eyre, Result, WrapErr};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct EchoProtocol {
  ver: u8,
  pub mtype: u8,
  pub msg: String,
}

impl EchoProtocol {
  pub fn to_json(&self) -> Result<String> {
    let pretty_json = serde_json::to_string_pretty(self)
      .wrap_err_with(|| eyre!("Problem serializing Message to JSON"))?;

    Ok(pretty_json)
  }

  // Define a method for future use
  #[allow(dead_code)]
  pub fn print_debug_msg(&self, msg: &str) {
    println!("<============ {}\\n {} \\n==============", 
      msg, self.to_json().unwrap()); 
  }

  // Define a method for future use
  #[allow(dead_code)]
  pub fn to_string(&self) -> String {
    format!("{:#?}", self)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  use pretty_assertions::assert_eq;

  #[test]
  fn constructor_sanity() {
    let message = EchoProtocol::new(1, 
      "Hello, world!".to_string());

    assert_eq!(message.mtype, 1);
  }
}
