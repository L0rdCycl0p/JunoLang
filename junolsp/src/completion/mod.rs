//This Source Code Form is subject to the terms of the Mozilla Public
//License, v. 2.0. If a copy of the MPL was not distributed with this
//file, You can obtain one at https://mozilla.org/MPL/2.0/.

use tower_lsp::{
    jsonrpc::Error,
    lsp_types::{CompletionParams, CompletionResponse},
};

use crate::backend::Backend;
mod file_global_completion;
mod get_program;
mod simple_completion;

pub(self) use get_program::get_program;
pub async fn completion(
    backend: &Backend,
    params: CompletionParams,
) -> Result<Option<CompletionResponse>, Error> {
    let mut items = simple_completion::keywords();
    items.append(&mut simple_completion::builtins());
    items.append(&mut simple_completion::snippets());
    items.append(
        &mut file_global_completion::file_global_completion(backend, params)
            .await
            .unwrap(),
    );
    Ok(Some(CompletionResponse::Array(items)))
}
