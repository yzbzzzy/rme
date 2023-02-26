use std::collections::HashMap;
use serde_derive::{Deserialize, Serialize};


#[derive(Debug, PartialEq, Eq, Deserialize,Serialize)]
pub struct response_data<T>{
    pub(crate) code:u32,
    pub(crate) data:HashMap<String,T>
}
