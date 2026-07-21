//This Source Code Form is subject to the terms of the Mozilla Public
//License, v. 2.0. If a copy of the MPL was not distributed with this
//file, You can obtain one at https://mozilla.org/MPL/2.0/.

use dashmap::DashMap;
use std::sync::Arc;
use tower_lsp::lsp_types::Url;

#[derive(Default)]
pub struct Workspace {
    files: DashMap<Url, String>,
}

impl Workspace {
    pub fn open(&self, uri: Url, text: String) {
        self.files.insert(uri, text);
    }

    pub fn update(&self, uri: Url, text: String) {
        self.files.insert(uri, text);
    }

    /*pub fn remove(&self, uri: &Url) {
        self.files.remove(uri);
    }*/

    pub fn source(&self, uri: &Url) -> Option<String> {
        self.files.get(uri).map(|f| f.clone())
    }
}

pub type SharedWorkspace = Arc<Workspace>;
