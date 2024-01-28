use crate::{cratesio::get_crate_info, text_store::get_text_document};
use anyhow::Result;
use lsp_types::{CompletionItem, CompletionParams, CompletionResponse};
use tracing::info;

enum CompletionKind {
    Versions,
    Features,
    None,
}

impl CompletionKind {
    fn should_handle_versions(line: &str) -> bool {
        line.ends_with("version = \"")
            || (!line.contains('{') && !line.contains('}') && line.ends_with("= \""))
    }

    fn should_handle_features(line: &str) -> bool {
        if !line.contains('[') {
            return false;
        }

        let features_endline = line.split_inclusive("[\"").next().unwrap();
        features_endline.ends_with("features = [\"")
    }
}

impl From<&str> for CompletionKind {
    fn from(line: &str) -> Self {
        if Self::should_handle_versions(line) {
            Self::Versions
        } else if Self::should_handle_features(line) {
            Self::Features
        } else {
            Self::None
        }
    }
}

fn get_crate_name(line: &str) -> &str {
    line.split_whitespace().next().unwrap()
}

async fn version_completions(line: &str) -> Result<CompletionResponse> {
    let crate_name = get_crate_name(line);
    let crate_info = get_crate_info(crate_name).await?;

    let completion_items = vec![CompletionItem {
        label: crate_info.version.clone(),
        ..Default::default()
    }];
    Ok(CompletionResponse::Array(completion_items))
}

async fn features_completions(line: &str) -> Result<CompletionResponse> {
    let crate_name = get_crate_name(line);
    let crate_info = get_crate_info(crate_name).await?;

    let completion_items = crate_info
        .features
        .iter()
        .map(|feature| CompletionItem {
            label: feature.clone(),
            ..Default::default()
        })
        .collect();
    Ok(CompletionResponse::Array(completion_items))
}

pub async fn handle_completion(params: CompletionParams) -> Result<CompletionResponse> {
    let text = get_text_document(params.text_document_position.text_document.uri).unwrap();
    let position = params.text_document_position.position;
    info!("Completion request for position: {:?}", position);
    let line: String = text
        .lines()
        .nth(position.line as usize)
        .unwrap()
        .trim()
        .chars()
        .take(position.character as usize)
        .collect();
    info!("Completion request for line: {}", line);

    match line.as_str().into() {
        CompletionKind::Versions => version_completions(&line).await,
        CompletionKind::Features => features_completions(&line).await,
        CompletionKind::None => Ok(CompletionResponse::Array(vec![])),
    }
}
