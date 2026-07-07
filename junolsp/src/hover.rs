use tower_lsp::{
    jsonrpc::Result,
    lsp_types::*,
};

use crate::backend::Backend;

pub async fn hover(
    _: &Backend,
    _: HoverParams,
) -> Result<Option<Hover>> {

    Ok(None)

}