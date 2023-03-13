use anyhow::anyhow;
use bytes::Bytes;
use serde_json::Value;
use std::{
    error::Error,
    str::{self, FromStr},
};

const REQUEST_ID_KEY: &str = "request_id";

pub fn try_get_request_id(payload: &Bytes) -> Result<String, Box<dyn Error>> {
    let payload_str = str::from_utf8(payload)?;
    let payload_val = Value::from_str(payload_str)?;

    let payload_obj = payload_val
        .as_object()
        .ok_or(anyhow!("expected payload to be object"))?;

    let request_id_val = payload_obj
        .get(&REQUEST_ID_KEY.to_string())
        .ok_or(anyhow!("expected object to have key {REQUEST_ID_KEY}"))?;

    Ok(request_id_val.to_string())
}
