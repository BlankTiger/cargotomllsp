use anyhow::Result;

fn main() -> Result<()> {
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
        definition_provider: Some(lsp_types::OneOf::Left(true)),
        ..Default::default()
    })?;
    eprintln!("Starting the server");
    let params = connection.initialize(server_capabilities)?;
    lsp_loop(connection, params)?;

    io_threads.join()?;
    eprintln!("Closing the server");
    Ok(())
}

fn lsp_loop(connection: lsp_server::Connection, params: serde_json::Value) -> Result<()> {
    eprintln!("Waiting for msgs...");
    for msg in &connection.receiver {
        eprintln!("Got a msg: {msg:?}");
        match msg {
            lsp_server::Message::Request(req) => {
                if connection.handle_shutdown(&req)? {
                    return Ok(());
                }

                match cast::<lsp_types::request::Completion>(req) {
                    Ok((id, params)) => eprintln!("Wow, {:?}: {:?}", id, params),
                    Err(err) => eprintln!("{:?}", err),
                }
            }
            lsp_server::Message::Notification(req) => {
                eprintln!("Yup, its a notification...");
                eprintln!("req: {:?}", req);
            }
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
