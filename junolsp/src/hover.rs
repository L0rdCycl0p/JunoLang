//This Source Code Form is subject to the terms of the Mozilla Public
//License, v. 2.0. If a copy of the MPL was not distributed with this
//file, You can obtain one at https://mozilla.org/MPL/2.0/.

use tower_lsp::{jsonrpc::Result, lsp_types::*};

use crate::backend::Backend;

pub async fn hover(_: &Backend, _: HoverParams) -> Result<Option<Hover>> {
    Ok(None)
}
