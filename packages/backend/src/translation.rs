use std::fs;

use axum::Json;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::composition::LlmResponse;

#[derive(Debug, Serialize, Deserialize)]
pub struct DoTranslation {
    src_json_model: String,
    tgt_json_language: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelTranslation {
    translated_json: String,
}

async fn translation_gemini_call(
    source_model_json: &String,
    target_theory: &String,
) -> Result<String, reqwest::Error> {
    let api_key = dotenvy::var("GEMINI_API_KEY").expect("GEMINI_API_KEY not set");

    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash:generateContent?key={}",
        api_key
    );

    let reasoning_url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-pro-exp-03-25:generateContent?key={}",
        api_key
    );

    let client = reqwest::Client::new();

    let system_instruction_text = match fs::read_to_string("trans_instruction0.txt") {
        Ok(instruction) => instruction,
        Err(_) => String::from("The default instruction"),
    };

    let model_translation_system_instruction = system_instruction_text;

    //let model_one_owned = model_one_json.to_owned();

    //let model_two_owned = model_two_json.to_owned();

    //let prompt_combined_json = model_one_json.to_owned().push_str(&model_two_json.to_owned());
    /*let prompt_text = format!(
        "Combine the following two models; Model A {model_one_owned}, Model B{model_two_owned}"
    );
    */

    let encoded_pdf = String::from("Hello this is a pdf");

    let src_model_owned = source_model_json.to_owned();
    let tgt_theory_owned = target_theory.to_owned();

    let prompt_text_many_models = format!(
        "Translate the following model: {src_model_owned} to a model in this: {tgt_theory_owned} target theory. Ensure that the Json Output conforms to the provided schema"
    );

    let model_composition_request_body = json!({
        "system_instruction": {
            "parts": {
                "text": model_translation_system_instruction
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
        //.post(&url)
        .post(&reasoning_url)
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
    println!("{response_text:#?}");
    println!("{one_candidate}");
    println!("{one_candidate_trimmed}");
    //Ok(response_text)
    //Ok(one_candidate.to_string())
    Ok(one_candidate_trimmed.to_string())
}

pub async fn translation_route(Json(payload): Json<DoTranslation>) -> Json<ModelTranslation> {
    let response_text =
        match translation_gemini_call(&payload.src_json_model, &payload.tgt_json_language).await {
            Ok(llm_composition) => llm_composition,
            Err(_e) => String::from("AI run into an error"),
        };

    let translation = ModelTranslation {
        translated_json: response_text,
    };
    Json(translation)
}
