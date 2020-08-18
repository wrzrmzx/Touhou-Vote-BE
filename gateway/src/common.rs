
use serde_derive::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Error {
    pub msg: String,
    pub aux: String
}

#[derive(Clone, Serialize, Deserialize)]
pub struct UserVerifyResult {
    pub user_id: String
}

#[derive(juniper::GraphQLObject, Clone, Serialize, Deserialize)]
pub struct PostResult {
    pub errno: i32
}

#[derive(juniper::GraphQLEnum, Clone, Serialize, Deserialize)]
pub enum VoteSection {
    Characters,
    Musics,
    CPs,
    Works,
    Papers
}

#[derive(juniper::GraphQLEnum, Clone, Serialize, Deserialize)]
pub enum FilterConditionOp {
    Equ,
    Neq,
    Gt,
    Gte,
    Lt,
    Lte,
    Contains
}

impl PostResult {
    pub fn new() -> PostResult {
        PostResult {
            errno: 0
        }
    }
}

macro_rules! getJSON {
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

macro_rules! postJSON {
    ($t:ident, $u:expr, $j:expr) => {
        {
            let client = reqwest::blocking::Client::new();
            let response = client.post(&$u).json(&$j).send()?;
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
