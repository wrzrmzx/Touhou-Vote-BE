
use serde_derive::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Error {
    pub msg: String,
    pub aux: String
}


macro_rules! getJSONResponseObj {
    ($t:ident, $u:expr) => {
        {
            let response = reqwest::blocking::get(&$u)?;
            if response.status().is_success() {
                let obj : $t = response.json()?;
                Ok(obj)
            } else {
                let e: Error = response.json()?;
                Err(
                    juniper::FieldError::new(
                        e.msg,
                        graphql_value!({
                            e.aux
                        }),
                    )
                )
            }
        }?
    };
}
