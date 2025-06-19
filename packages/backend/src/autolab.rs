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

#[derive(Deserialize)]
pub struct DoThreadSuggestion {
    lib_models: Vec<String>,
    current_model: String,
    thread_primer: String,
}

#[derive(Deserialize)]
pub struct DoThreadComposition {
    active_model: String,
    lib_models: Vec<String>,
    selected_thread: String,
}

// the output of our composition handler
#[derive(Serialize)]
pub struct ModelThreads {
    suggested_threads: Vec<String>,
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

#[derive(Debug, Deserialize)]
pub struct ThreadSuggestions {
    threads: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Suggestion {
    thread: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DoTranslation {
    src_json_model: String,
    tgt_json_language: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelTranslation {
    translated_json: String,
}

async fn make_ollama_call(
    models_json: &[String],
    application: &String,
) -> Result<String, reqwest::Error> {
    Ok(String::from("Some fancy llm response"))
}

async fn composition_gemini_call(
    models_json: &[String],
    application: &String,
) -> Result<String, reqwest::Error> {
    let api_key = dotenvy::var("GEMINI_API_KEY").expect("GEMINI_API_KEY not set");

    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash:generateContent?key={}",
        api_key
    );

    let thinking_url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash-thinking-exp-01-21:generateContent?key={}",
        api_key
    );

    let reasoning_url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-pro-exp-03-25:generateContent?key={}",
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

    dbg!(std::env::current_dir().unwrap());

    let system_instruction_text = match fs::read_to_string(
        "/home/amati/CodeProjects/Autostructures/code/Live-catcolab/code/CatColab/packages/backend/src/system/compositioninstruction.txt",
    ) {
        Ok(instruction) => instruction,
        Err(e) => format!("The default instruction: {e}"),
    };

    let model_composition_system_instruction = system_instruction_text;

    //let model_one_owned = model_one_json.to_owned();

    //let model_two_owned = model_two_json.to_owned();

    //let prompt_combined_json = model_one_json.to_owned().push_str(&model_two_json.to_owned());
    /*let prompt_text = format!(
        "Combine the following two models; Model A {model_one_owned}, Model B{model_two_owned}"
    );
    */

    let encoded_pdf = String::from("Hello this is a pdf");

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
            //"response_mime_type": "application/json",
            //"response_schema": {
                //"type": "ARRAY",
                //"items": {
                  //  "type": "OBJECT",
                   // "properties": {
                     //   "thread": {"type":"STRING"}
                    //}
                //}
            //}
        }
    });

    let response: serde_json::Value = client
        //.post(&url)
        //.post(&reasoning_url)
        .post(&thinking_url)
        .json(&model_composition_request_body)
        .send()
        .await?
        .json()
        .await?;

    let response_text = response.to_string();
    println!("{response_text:#?}");
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

async fn translation_gemini_call(
    source_model_json: &String,
    target_theory: &String,
) -> Result<String, reqwest::Error> {
    let api_key = dotenvy::var("GEMINI_API_KEY").expect("GEMINI_API_KEY not set");

    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash:generateContent?key={}",
        api_key
    );

    let thinking_url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash-thinking-exp-01-21:generateContent?key={}",
        api_key
    );

    let reasoning_url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-pro-exp-03-25:generateContent?key={}",
        api_key
    );

    let client = reqwest::Client::new();

    let system_instruction_text = match fs::read_to_string(
        "/home/amati/CodeProjects/Autostructures/code/Live-catcolab/code/CatColab/packages/backend/src/system/translation_instruction.txt",
    ) {
        Ok(instruction) => instruction,
        Err(_) => String::from("The default instruction"),
    };

    let model_translation_system_instruction = system_instruction_text;

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
        .post(&url)
        //.post(&reasoning_url)
        //.post(&thinking_url)
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

async fn thread_suggestion_gemini_call(
    current_model: &String,
    lib_models_json: &[String],
    user_defined_suggestion: &String,
) -> Result<Vec<String>, reqwest::Error> {
    let api_key = dotenvy::var("GEMINI_API_KEY").expect("GEMINI_API_KEY not set");

    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash:generateContent?key={}",
        api_key
    );

    let thinking_url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash-thinking-exp-01-21:generateContent?key={}",
        api_key
    );

    let reasoning_url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-pro-exp-03-25:generateContent?key={}",
        api_key
    );

    let client = reqwest::Client::new();

    let system_instruction_text = match fs::read_to_string("system/thread_instruction0.txt") {
        Ok(instruction) => instruction,
        Err(_) => String::from("The default instruction"),
    };

    let model_thread_suggestion_system_instruction = system_instruction_text;

    let encoded_pdf = String::from("Hello this is a pdf");

    let lib_models_owned = lib_models_json.concat().to_owned();
    let current_model_owned = current_model.to_owned();

    let prompt_text_for_suggestions = format!(
        "Suggest possible enhancement options for the following model: {current_model_owned}. The suggestions should be a list of Strings"
    );

    let thread_suggestion_request_body = json!({
        "system_instruction": {
            "parts": {
                "text": model_thread_suggestion_system_instruction
            }

        },
        "contents": [{
            "parts": [
                    {"text": prompt_text_for_suggestions}
            ]
        }],
        "generationConfig": {
            "response_mime_type": "application/json",
            "response_schema": {
                "type": "ARRAY",
                "items": {
                    "type": "OBJECT",
                    "properties": {
                        "thread": {"type":"STRING"}
                    }
                }
            }
        }
    });

    let response: serde_json::Value = client
        .post(&url)
        //.post(&reasoning_url)
        //.post(&thinking_url)
        .json(&thread_suggestion_request_body)
        .send()
        .await?
        .json()
        .await?;

    let response_text = response.to_string();
    println!("{response_text:#?}");
    let response_parsed: LlmResponse = serde_json::from_str(response_text.as_str()).unwrap();
    let one_candidate = &response_parsed.candidates[0].content.parts[0].text;
    let thread_suggestions: Vec<Suggestion> = serde_json::from_str(one_candidate).unwrap();
    let one_candidate_trimmed =
        one_candidate.trim_start_matches('[').trim_end_matches(']').to_string();
    println!("{response_text:#?}");
    println!("{one_candidate}");
    println!("{one_candidate_trimmed}");
    let thread_strings: Vec<String> = thread_suggestions.into_iter().map(|s| s.thread).collect();
    println!("{thread_strings:#?}");
    Ok(thread_strings)
}

async fn thread_composition_gemini_call(
    active_model: &String,
    lib_models: &[String],
    selected_thread: &String,
) -> Result<String, reqwest::Error> {
    let api_key = dotenvy::var("GEMINI_API_KEY").expect("GEMINI_API_KEY not set");

    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash:generateContent?key={}",
        api_key
    );

    let thinking_url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash-thinking-exp-01-21:generateContent?key={}",
        api_key
    );

    let reasoning_url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-pro-exp-03-25:generateContent?key={}",
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

    let system_instruction_text = match fs::read_to_string("system/thread_composition0.txt") {
        Ok(instruction) => instruction,
        Err(_) => String::from("The default instruction"),
    };

    let model_composition_system_instruction = system_instruction_text;

    let encoded_pdf = String::from("Hello this is a pdf");

    let models_owned = lib_models.concat().to_owned();
    let enhancement_thread = selected_thread.to_owned();
    let active_model_owned = active_model.to_owned();

    let prompt_text_many_models = format!(
        "Enhance the following model {active_model_owned} using the thread {enhancement_thread}. Use the following library Models: {models_owned} as source material and ensure that the enhancement only includes content that is similar to {enhancement_thread}."
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
            //"response_mime_type": "application/json",
            //"response_schema": {
                //"type": "ARRAY",
                //"items": {
                  //  "type": "OBJECT",
                   // "properties": {
                     //   "thread": {"type":"STRING"}
                    //}
                //}
            //}
        }
    });

    let response: serde_json::Value = client
        .post(&url)
        //.post(&reasoning_url)
        //.post(&thinking_url)
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

pub async fn composition_route(Json(payload): Json<DoComposition>) -> Json<ModelComposition> {
    let response_text = match composition_gemini_call(&payload.models, &payload.application).await {
        Ok(llm_composition) => llm_composition,
        Err(_e) => String::from("AI run into an error"),
    };
    let composition = ModelComposition {
        composition_text: response_text,
    };

    Json(composition)
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

pub async fn thread_suggestion_route(
    Json(payload): Json<DoThreadSuggestion>,
) -> Json<ModelThreads> {
    let model_thread_suggestions = match thread_suggestion_gemini_call(
        &payload.current_model,
        &payload.lib_models,
        &payload.thread_primer,
    )
    .await
    {
        Ok(thread_suggestions) => thread_suggestions,
        Err(_e) => [
            String::from("Default suggestion one"),
            String::from("Default suggestion two"),
            String::from("Default suggestion 3"),
        ]
        .to_vec(),
    };

    let threads = ModelThreads {
        suggested_threads: model_thread_suggestions,
    };

    Json(threads)
}

pub async fn thread_composition_route(
    Json(payload): Json<DoThreadComposition>,
) -> Json<ModelComposition> {
    let thread_composition = match thread_composition_gemini_call(
        &payload.active_model,
        &payload.lib_models,
        &payload.selected_thread,
    )
    .await
    {
        Ok(thread_composition) => thread_composition,
        Err(_e) => String::from("This is an example of thread composition!"),
    };

    let composition = ModelComposition {
        composition_text: thread_composition,
    };

    Json(composition)
}
