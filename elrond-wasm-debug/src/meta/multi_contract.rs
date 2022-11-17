
use std::collections::HashMap;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct MultiContract{
    pub settings: MCSettings,
    pub contracts: HashMap<String, MCContract>,
    pub labels: MCLabel,
}

#[derive(Deserialize, Debug)]
pub struct MCLabel{
    pub default: Vec<String>,
    pub ev: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct MCContract{
    pub externalview: Option<bool>,
    pub wasmname: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct MCSettings{
    pub default: String,
}

