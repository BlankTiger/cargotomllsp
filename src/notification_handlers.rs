use super::text_store::TEXT_STORE;
use tracing::info;

pub fn handle_doc_change(notification: lsp_server::Notification) {
    let params = notification
        .extract::<lsp_types::DidChangeTextDocumentParams>("textDocument/didChange")
        .unwrap();
    let uri = params.text_document.uri;
    let text = params.content_changes[0].text.to_string();

    if params.content_changes.len() > 1 {
        info!("More than one content change, please be wary");
    }

    info!("Did change: {:?}", uri);
    TEXT_STORE
        .get()
        .expect("text store not initialized")
        .lock()
        .expect("text store mutex poisoned")
        .insert(uri.to_string(), text);
}

pub fn handle_doc_open(notification: lsp_server::Notification) {
    let params = notification
        .extract::<lsp_types::DidOpenTextDocumentParams>("textDocument/didOpen")
        .unwrap();
    let uri = params.text_document.uri;
    let text = params.text_document.text.to_string();

    info!("Did open: {:?}", uri);
    TEXT_STORE
        .get()
        .expect("text store not initialized")
        .lock()
        .expect("text store mutex poisoned")
        .insert(uri.to_string(), text);
}
