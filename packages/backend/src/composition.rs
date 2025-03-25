use std::fs;

use axum::Json;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Deserialize)]
pub struct DoComposition {
    models: Vec<String>,
    application: String,
}

// the output of our composition handler
#[derive(Serialize)]
pub struct ModelComposition {
    composition_text: String,
}

#[derive(Debug, Deserialize)]
pub struct LlmResponse {
    pub candidates: Vec<Candidate>,
    pub model_version: Option<String>,
    pub usage_metadata: Option<UsageMetadata>,
}

#[derive(Debug, Deserialize)]
pub struct Candidate {
    pub avg_log_probs: Option<f64>,
    pub content: Content,
    pub finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Content {
    pub parts: Vec<Part>,
    pub role: String,
}

#[derive(Debug, Deserialize)]
pub struct Part {
    pub text: String,
}

#[derive(Debug, Deserialize)]
pub struct UsageMetadata {
    pub candidates_token_count: u32,
    pub candidates_tokens_details: Vec<TokenDetail>,
    pub prompt_token_count: u32,
    pub prompt_tokens_details: Vec<TokenDetail>,
    pub total_token_count: u32,
}

#[derive(Debug, Deserialize)]
pub struct TokenDetail {
    pub modality: String,
    pub token_count: u32,
}

async fn make_llm_call(
    models_json: &[String],
    application: &String,
) -> Result<String, reqwest::Error> {
    let api_key = dotenvy::var("GEMINI_API_KEY").expect("GEMINI_API_KEY not set");

    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash:generateContent?key={}",
        api_key
    );

    let client = reqwest::Client::new();
    let _request_body = json!({
        "system_instruction": [{
            "parts": [
                {"text": "You are a very good chef. Your name is Bruno."}
            ]
        }],
        "contents": [{
            "parts": [
                {"text": "List 5 popular cookie recipes"}
            ]
        }],
        "generationConfig": {
            "response_mime_type": "application/json",
            "response_schema": {
                "type": "ARRAY",
                "items": {
                    "type": "OBJECT",
                    "properties": {
                        "recipe_name": {"type":"STRING"}
                    }
                }
            }
        }
    });

    let system_instruction_text = match fs::read_to_string("instruction.txt") {
        Ok(instruction) => instruction,
        Err(_) => String::from("The default instruction"),
    };

    let model_composition_system_instruction = system_instruction_text;

    //let model_one_owned = model_one_json.to_owned();

    //let model_two_owned = model_two_json.to_owned();

    //let prompt_combined_json = model_one_json.to_owned().push_str(&model_two_json.to_owned());
    /*let prompt_text = format!(
        "Combine the following two models; Model A {model_one_owned}, Model B{model_two_owned}"
    );
    */

    let models_owned = models_json.concat().to_owned();
    let application_owned = application.to_owned();

    let prompt_text_many_models = format!(
        "Combine the following models, Models: {models_owned} so as to be insighful in this: {application_owned} application"
    );

    let model_composition_request_body = json!({
        "system_instruction": {
            "parts": {
                "text": model_composition_system_instruction
            }

        },
        "contents": [{
            "parts": [
                    {"text": prompt_text_many_models}
            ]
        }],
        "generationConfig": {
            "response_mime_type": "application/json"
            //"response_schema": {
                //"type": "ARRAY",
                //"items": {
                   // "type": "OBJECT",
                   // "properties": {
                  //      "recipe_name": {"type":"STRING"}
                //    }
              //  }
            //}
        }
    });

    let response: serde_json::Value = client
        .post(&url)
        .json(&model_composition_request_body)
        .send()
        .await?
        .json()
        .await?;

    let response_text = response.to_string();
    let response_parsed: LlmResponse = serde_json::from_str(response_text.as_str()).unwrap();
    let one_candidate = &response_parsed.candidates[0].content.parts[0].text;
    let one_candidate_trimmed =
        one_candidate.trim_start_matches('[').trim_end_matches(']').to_string();
    //println!("{response_parsed:#?}");
    //println!("{response:#?}");
    println!("{one_candidate}");
    println!("{one_candidate_trimmed}");
    //Ok(response_text)
    //Ok(one_candidate.to_string())
    Ok(one_candidate_trimmed.to_string())
}

pub async fn composition_route(Json(payload): Json<DoComposition>) -> Json<ModelComposition> {
    let response_text = match make_llm_call(&payload.models, &payload.application).await {
        Ok(llm_composition) => llm_composition,
        Err(_e) => String::from("AI run into an error"),
    };
    let composition = ModelComposition {
        composition_text: response_text,
    };

    Json(composition)
}
