use tower_lsp::{ jsonrpc::Result, lsp_types::* };
use libjuno::{ BuiltinEnum, REGISTRY };
use crate::backend::Backend;

pub async fn completion(
    _backend: &Backend,
    _params: CompletionParams
) -> Result<Option<CompletionResponse>> {
    let mut items = keywords();
    items.append(&mut builtins());
    Ok(Some(CompletionResponse::Array(items)))
}

fn builtins() -> Vec<CompletionItem> {
    let mut items = vec![];

    for (name, i) in REGISTRY.entries().into_iter() {
        items.push(match i.declare {
            BuiltinEnum::Function { param_types: _, return_type: _ } =>
                CompletionItem {
                    label: (*name).into(),
                    kind: Some(CompletionItemKind::FUNCTION),
                    detail: Some("Builtin".into()),
                    ..Default::default()
                },
        });
    }
    items
}

fn keywords() -> Vec<CompletionItem> {
    vec![
        CompletionItem {
            label: "fn".into(),
            kind: Some(CompletionItemKind::KEYWORD),
            detail: Some("Function".into()),
            ..Default::default()
        },

        CompletionItem {
            label: "let".into(),
            kind: Some(CompletionItemKind::KEYWORD),
            detail: Some("Variable".into()),
            ..Default::default()
        },
        CompletionItem {
            label: "mut".into(),
            kind: Some(CompletionItemKind::KEYWORD),
            detail: Some("Variable".into()),
            ..Default::default()
        },
        CompletionItem {
            label: "if".into(),
            kind: Some(CompletionItemKind::KEYWORD),
            detail: Some("Variable".into()),
            ..Default::default()
        },
        CompletionItem {
            label: "else".into(),
            kind: Some(CompletionItemKind::KEYWORD),
            detail: Some("Variable".into()),
            ..Default::default()
        },
        CompletionItem {
            label: "while".into(),
            kind: Some(CompletionItemKind::KEYWORD),
            detail: Some("Variable".into()),
            ..Default::default()
        },
        CompletionItem {
            label: "for".into(),
            kind: Some(CompletionItemKind::KEYWORD),
            detail: Some("Variable".into()),
            ..Default::default()
        },
        CompletionItem {
            label: "return".into(),
            kind: Some(CompletionItemKind::KEYWORD),
            detail: Some("Variable".into()),
            ..Default::default()
        },
        CompletionItem {
            label: "break".into(),
            kind: Some(CompletionItemKind::KEYWORD),
            detail: Some("Variable".into()),
            ..Default::default()
        },
        CompletionItem {
            label: "continue".into(),
            kind: Some(CompletionItemKind::KEYWORD),
            detail: Some("Variable".into()),
            ..Default::default()
        },
        CompletionItem {
            label: "import".into(),
            kind: Some(CompletionItemKind::KEYWORD),
            detail: Some("Variable".into()),
            ..Default::default()
        },

        CompletionItem {
            label: "struct".into(),
            kind: Some(CompletionItemKind::KEYWORD),
            detail: Some("Struct".into()),
            ..Default::default()
        }
    ]
}
