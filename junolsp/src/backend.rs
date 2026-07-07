use std::sync::Arc;

use tower_lsp::{ jsonrpc::Result, lsp_types::*, Client, LanguageServer };

use crate::{
    completion,
    diagnostics,
    goto_definition,
    hover,
    semantic_tokens,
    workspace::{ SharedWorkspace, Workspace },
};

pub struct Backend {
    pub client: Client,
    pub workspace: SharedWorkspace,
}

impl Backend {
    pub fn new(client: Client) -> Self {
        Self {
            client,
            workspace: Arc::new(Workspace::default()),
        }
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(
                    TextDocumentSyncCapability::Kind(TextDocumentSyncKind::FULL)
                ),

                completion_provider: Some(CompletionOptions::default()),

                hover_provider: Some(HoverProviderCapability::Simple(true)),

                definition_provider: Some(OneOf::Left(true)),

                semantic_tokens_provider: Some(
                    SemanticTokensServerCapabilities::SemanticTokensOptions(
                        SemanticTokensOptions::default()
                    )
                ),

                ..Default::default()
            },

            ..Default::default()
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client.log_message(MessageType::INFO, "Juno Language Server startet.").await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        self.client.log_message(
            MessageType::INFO,
            format!("Opened {}", params.text_document.uri)
        ).await;

        self.workspace.open(params.text_document.uri, params.text_document.text);
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        if let Some(change) = params.content_changes.first() {
            self.client.log_message(
                MessageType::INFO,
                format!("Changed ({} bytes)", change.text.len())
            ).await;

            self.workspace.update(params.text_document.uri.clone(), change.text.clone());
        }
    }

    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        completion::completion(self, params).await
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        hover::hover(self, params).await
    }
}
