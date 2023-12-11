use api_macro::GetAttributesMacro;
use api_macro_derive::GetAttributesMacro;
use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    #[serde(rename = "project_id")]
    pub project_id: String,
    pub name: String,
    pub description: String,
    pub asset: Asset,
    pub online: Online,
    pub tags: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, GetAttributesMacro)]
#[serde(rename_all = "camelCase")]
pub struct Asset {
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, GetAttributesMacro)]
#[serde(rename_all = "camelCase")]
pub struct Online {
    pub parameters: Parameters,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, GetAttributesMacro)]
#[serde(rename_all = "camelCase")]
pub struct Parameters {
    #[serde(rename = "serving_name")]
    pub serving_name: String,
}
