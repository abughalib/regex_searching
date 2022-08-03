use serde::{Serialize, Deserialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    pub code: String,
    pub no_ocr: Vec<Value>,
    pub ocrs: Option<Value>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ocrs{
    #[serde(rename="detectedLanguages")]
    detected_languages: Vec<DetectedLanguages>,
    text: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DetectedLanguages{
    confidence: f32,
    #[serde(rename="languageCode")]
    language_code: String,
}