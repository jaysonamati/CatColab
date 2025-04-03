use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DoTranslation {
    src_json_model: String,
    tgt_json_language: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelTranslation {
    translated_json: String,
}

pub async fn translation_route(Json(payload): Json<DoTranslation>) -> Json<ModelTranslation> {
    /*let response_text = match make_gemini_call(&payload.models, &payload.application).await {
            Ok(llm_composition) => llm_composition,
            Err(_e) => String::from("AI run into an error"),
        };
    */
    let translation = ModelTranslation {
        translated_json: String::from("An example translation"),
    };
    Json(translation)
}
