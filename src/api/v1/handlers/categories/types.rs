use super::super::prelude::types::*;


#[derive(Serialize, Deserialize)]
pub struct CreateRootCatgoriePayload{
    pub name: String
}

#[derive(Serialize, Deserialize)]
pub struct CreateInnerCatgoriePayload{
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateInnerInnerCatgoriePayload{
    pub name: String,
}
