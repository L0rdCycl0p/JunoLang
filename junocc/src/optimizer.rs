//This Source Code Form is subject to the terms of the Mozilla Public
//License, v. 2.0. If a copy of the MPL was not distributed with this
//file, You can obtain one at https://mozilla.org/MPL/2.0/.

use libjuno::inkwell::{module::Module, passes::PassBuilderOptions};

use crate::get_target_machine;

pub fn optimize(module: &mut Module) {
    let target_machine = get_target_machine();
    module
        .run_passes("default<O3>", &target_machine, PassBuilderOptions::create())
        .unwrap();
}
