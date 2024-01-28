use super::text_store::TEXT_STORE;
use std::io::Write;

pub fn handle_doc_change(req: lsp_server::Notification) {
    let params = req
        .extract::<lsp_types::DidChangeTextDocumentParams>("textDocument/didChange")
        .unwrap();
    let uri = params.text_document.uri;
    let text = params.content_changes[0].text.to_string();

    if params.content_changes.len() > 1 {
        eprintln!("More than one content change, please be wary");
    }

    let mut file = std::fs::File::create("/home/blanktiger/cargotomllsp.toml").unwrap();
    file.write_all(text.as_bytes()).unwrap();
    eprintln!("Did change: {:?}", uri);
    TEXT_STORE
        .get()
        .expect("text store not initialized")
        .lock()
        .expect("text store mutex poisoned")
        .insert(uri.to_string(), text);
}
