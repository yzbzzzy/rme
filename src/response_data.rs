use std::collections::HashMap;
use serde_derive::{Deserialize, Serialize};


#[derive(Debug, PartialEq, Eq, Deserialize,Serialize)]
pub struct ResponseData<T>{
    pub(crate) code:u8,
    pub(crate) data:HashMap<String,T>
}
