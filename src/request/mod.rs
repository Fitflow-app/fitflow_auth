use color_eyre::Result;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// A structure to provide type-safety when communicating with the API.
/// Once serialized, you can send using the **POST** method at `auth.fitflow.app`.
///
/// ```
/// # use fitflow_auth::Registration;
/// #
/// let r = Registration::new(
///     "email@domain.com".into(),
///     "Hello".into(),
///     "WORLD".into(),
///     "mySecurePassword11*".into(),
/// ).unwrap();
/// // This can be serialized and sent to `auth.fitflow.app`
/// ```
#[derive(Validate, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Registration {
    #[validate(email, length(min = 6, max = 128))]
    pub email: String,

    #[validate(length(code = "first_name", min = 2, max = 32))]
    pub first_name: String,

    #[validate(length(code = "last_name", min = 2, max = 32))]
    pub last_name: String,

    #[validate(length(code = "password", min = 8, max = 64))]
    pub password: String,
}

impl Registration {
    /// Creates a new [`Registration`] and runs the validation over its
    /// attributes.
    pub fn new(
        email: String,
        first_name: String,
        last_name: String,
        password: String,
    ) -> Result<Registration> {
        let r = Registration {
            email,
            first_name,
            last_name,
            password,
        };
        r.validate()?;
        Ok(r)
    }
}

/// A structure to provide type-safety when communicating with the API.
/// Once serialized, you can send using the **GET** method at `auth.fitflow.app`.
///
/// ```
/// # use fitflow_auth::Login;
/// #
/// let r = Login::new(
///     "email@domain.com".into(),
///     "mySecurePassword11*".into(),
/// ).unwrap();
/// // This can be serialized and sent to `auth.fitflow.app`
/// ```
#[derive(Validate, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Login {
    #[validate(email, length(min = 6, max = 128))]
    pub email: String,

    #[validate(length(code = "password", min = 8, max = 64))]
    pub password: String,
}

impl Login {
    /// Creates a new [`Login`] and runs the validation over its
    /// attributes.
    pub fn new(email: String, password: String) -> Result<Login> {
        let r = Login { email, password };
        r.validate()?;
        Ok(r)
    }
}
