// src/openai.rs
use reqwest::{Client, multipart};
use serde::{Deserialize, Serialize};
use std::path::Path;
use tokio::fs::File;
use tokio_util::codec::{BytesCodec, FramedRead};

// ... (Structs InputMessage, ResponsesRequest, etc. iguales que antes) ...
#[derive(Serialize, Clone)]
pub struct InputMessage {
    pub role: String,
    pub content: String,
}

#[derive(Serialize)]
pub struct ResponsesRequest {
    pub model: String,
    pub input: Vec<InputMessage>,
    pub temperature: f32,
    pub store: bool,
}

#[derive(Deserialize, Default)]
pub struct ResponsesResponse {
    pub output: Vec<OutputItem>,
}

#[derive(Deserialize, Default)]
pub struct OutputItem {
    #[serde(rename = "type")]
    pub item_type: String, 
    pub content: Option<Vec<ContentPart>>,
}

#[derive(Deserialize, Default)]
pub struct ContentPart {
    #[serde(rename = "type")]
    pub part_type: String, 
    pub text: Option<String>,
}

#[derive(Deserialize)]
pub struct WhisperResponse {
    pub text: String,
}

pub fn extract_output_text(resp: &ResponsesResponse) -> Option<String> {
    for item in &resp.output {
        if item.item_type == "message" {
            if let Some(parts) = &item.content {
                for p in parts {
                    if p.part_type == "output_text" {
                        if let Some(t) = &p.text {
                            return Some(t.clone());
                        }
                    }
                }
            }
        }
    }
    None
}

pub async fn call_responses_api(
    client: &Client,
    api_key: &str,
    req: &ResponsesRequest,
) -> Result<ResponsesResponse, String> {
    let resp = client
        .post("https://api.openai.com/v1/responses")
        .bearer_auth(api_key)
        .json(req)
        .send()
        .await
        .map_err(|e| format!("Error OpenAI API: {e}"))?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        return Err(format!("Error OpenAI ({status}): {body}"));
    }

    resp.json::<ResponsesResponse>().await.map_err(|e| e.to_string())
}

// CORRECCIÓN PARA AUDIO
pub async fn transcribe_audio(
    client: &Client,
    api_key: &str,
    file_path: &str,
) -> Result<String, String> {
    let path = Path::new(file_path);
    // Extraemos nombre, pero si no tiene extensión válida, Whisper puede fallar.
    // En el frontend ya aseguramos .webm o .m4a
    let filename = path.file_name().unwrap().to_string_lossy().to_string();

    let file = File::open(path).await.map_err(|e| format!("No se pudo abrir audio: {e}"))?;
    
    // Usamos FramedRead para streaming eficiente
    let stream = FramedRead::new(file, BytesCodec::new());
    let file_part = multipart::Part::stream(reqwest::Body::wrap_stream(stream))
        .file_name(filename.clone())
        .mime_str("audio/webm") // Ponemos webm por defecto, Whisper es tolerante
        .map_err(|e| e.to_string())?;

    let form = multipart::Form::new()
        .text("model", "whisper-1")
        .part("file", file_part);

    let resp = client
        .post("https://api.openai.com/v1/audio/transcriptions")
        .bearer_auth(api_key)
        .multipart(form)
        .send()
        .await
        .map_err(|e| format!("Error enviando audio a Whisper: {e}"))?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        return Err(format!("Error Whisper ({status}): {body}"));
    }

    let parsed: WhisperResponse = resp.json().await.map_err(|e| format!("Error JSON Whisper: {e}"))?;
    Ok(parsed.text)
}
