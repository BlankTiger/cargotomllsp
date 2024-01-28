use anyhow::Result;
use cargotomllsp::{
    completion_handler::handle_completion,
    cratesio::init_crate_store,
    logging::setup_logging,
    notification_handlers::{handle_doc_change, handle_doc_open},
    text_store::init_text_store,
};
use tracing::{error, info, warn};

fn setup_stores() {
    init_crate_store();
    init_text_store();
}

async fn lsp_loop(connection: lsp_server::Connection, _params: serde_json::Value) -> Result<()> {
    for msg in &connection.receiver {
        match msg {
            lsp_server::Message::Request(req) => {
                if connection.handle_shutdown(&req)? {
                    return Ok(());
                }

                match cast::<lsp_types::request::Completion>(req) {
                    Ok((id, params)) => {
                        let completion_response =
                            match serde_json::to_value(&handle_completion(params).await?) {
                                Ok(value) => value,
                                Err(err) => {
                                    info!("{:?}", err);
                                    serde_json::Value::Null
                                }
                            };
                        connection.sender.send(lsp_server::Message::Response(
                            lsp_server::Response {
                                id,
                                result: Some(completion_response),
                                error: None,
                            },
                        ))?
                    }
                    Err(err) => error!("{:?}", err),
                };
            }

            lsp_server::Message::Notification(notification) => match notification.method.as_str() {
                "textDocument/didSave" => {
                    info!("Did save: {:?}", notification);
                }
                "textDocument/didOpen" => handle_doc_open(notification),
                "textDocument/didChange" => handle_doc_change(notification),
                _ => warn!("Unhandled notification: {:?}", notification),
            },

            _ => {}
        }
    }
    Ok(())
}

fn cast<R>(
    req: lsp_server::Request,
) -> Result<(lsp_server::RequestId, R::Params), lsp_server::ExtractError<lsp_server::Request>>
where
    R: lsp_types::request::Request,
    R::Params: serde::de::DeserializeOwned,
{
    req.extract(R::METHOD)
}

#[tokio::main]
async fn main() -> Result<()> {
    setup_logging()?;
    setup_stores();

    let (connection, io_threads) = lsp_server::Connection::stdio();
    let server_capabilities = serde_json::to_value(lsp_types::ServerCapabilities {
        completion_provider: Some(lsp_types::CompletionOptions {
            resolve_provider: Some(true),
            trigger_characters: Some(vec!["\"".to_string()]),
            all_commit_characters: None,
            completion_item: Some(lsp_types::CompletionOptionsCompletionItem {
                label_details_support: Some(true),
            }),
            work_done_progress_options: lsp_types::WorkDoneProgressOptions {
                work_done_progress: None,
            },
        }),
        // definition_provider: Some(lsp_types::OneOf::Left(true)),
        text_document_sync: Some(lsp_types::TextDocumentSyncCapability::Kind(
            lsp_types::TextDocumentSyncKind::FULL,
        )),
        ..Default::default()
    })?;

    info!("Starting the server");
    let params = connection.initialize(server_capabilities)?;
    lsp_loop(connection, params).await?;

    io_threads.join()?;
    info!("Closing the server");
    Ok(())
}
