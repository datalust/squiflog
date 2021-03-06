use std::{borrow::Cow, collections::HashMap};
use serde_json::Value;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct Message<'a> {
    #[serde(rename = "@t")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "@l")]
    pub level: Option<&'a str>,

    #[serde(rename = "@m")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<Cow<'a, str>>,

    // @mt and @x are currently not used
    #[serde(rename = "@mt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_template: Option<&'a str>,

    #[serde(rename = "@x")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exception: Option<&'a str>,

    // @i and @r are currently not implemented

    // Everything else
    #[serde(flatten)]
    pub additional: HashMap<&'a str, Value>,
}
