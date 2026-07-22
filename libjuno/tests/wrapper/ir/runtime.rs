use super::*;
use crate::wrapper::ir::utils::{make_backend, test_program};

#[test]
fn declare_runtime_is_no_op() {
    let prog = Box::leak(Box::new(test_program()));
    let (mut backend, _ctx) = make_backend(prog);
    backend.declare_runtime(); // should not panic
}
