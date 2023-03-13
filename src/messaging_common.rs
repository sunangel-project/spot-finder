use bytes::Bytes;
use std::{error::Error, str};

pub fn try_get_request_id(payload: &Bytes) -> Result<String, Box<dyn Error>> {
    // TODO: get this monstrosity cleaner
    /*let request_id = str::from_utf8(&payload)
    .map(|payload| {
        serde_json::to_value(payload)
            .map(|value| {
                value
                    .as_object()
                    .and_then(|object| object.get(&"request_id".to_string()))
                    .map(|value| value.to_string())
            })
            .ok()
    })
    .ok()
    .flatten()
    .flatten();*/

    let payload_str = str::from_utf8(payload)?;

    Ok("UNKNOWN".to_string())
}
