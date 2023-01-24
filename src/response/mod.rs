use color_eyre::Result;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// A structure to provide type-safety when communicating with the API.
/// [`Token`] is the response (if no error occurred) to login/registration request.
///
/// ```
/// # use fitflow_auth::response::Token;
/// #
/// # let key = "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA";
/// let r = Token::new(key.into()).unwrap();
/// // This can be serialized and sent to `auth.fitflow.app`
/// ```
#[derive(Validate, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Token {
    #[validate(length(equal = 256))]
    pub key: String,
}

impl Token {
    /// Creates a new [`Token`] and runs the validation over its
    /// attributes.
    pub fn new(key: String) -> Result<Token> {
        let r = Token { key };
        r.validate()?;
        Ok(r)
    }
}
