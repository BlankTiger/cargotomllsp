use crate::cratesio::get_crate_info;
use crate::text_store::get_text_document;
use anyhow::Result;
use lsp_types::{CompletionItem, CompletionParams, CompletionResponse};

pub async fn handle_completion(params: CompletionParams) -> Result<CompletionResponse> {
    let text = get_text_document(params.text_document_position.text_document.uri).unwrap();
    let position = params.text_document_position.position;
    let line = text.lines().nth(position.line as usize).unwrap().trim();

    if !line.ends_with("version = \"") && !line.ends_with("= \"") {
        return Ok(CompletionResponse::Array(vec![]));
    }

    let crate_name = line.split_whitespace().next().unwrap();
    let crate_info = get_crate_info(crate_name).await?;

    let completion_items = vec![CompletionItem {
        label: crate_info.version.clone(),
        ..Default::default()
    }];
    Ok(CompletionResponse::Array(completion_items))
}
