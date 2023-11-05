use super::*;
use lsp_types::{
    InlayHint, InlayHintKind, InlayHintParams, Position, Range, TextDocumentIdentifier, TextEdit,
    Url,
};

fn expect_hints(src: &str, expected_hints: Vec<InlayHint>) {
    let hints = inlay_hints(src).expect("should return hints");

    // InlayHint doesn't implement PartialEq so we're serialising to compare them
    let hints = serde_json::to_value(hints).expect("serialisation shouldn't fail");
    let expected_hints =
        serde_json::to_value(expected_hints).expect("serialisation shouldn't fail");
    assert_eq!(hints, expected_hints);
}

fn inlay_hints(src: &str) -> Option<Vec<InlayHint>> {
    let io = LanguageServerTestIO::new();
    let mut engine = setup_engine(&io);

    _ = io.src_module("app", src);
    let response = engine.compile_please();
    assert!(response.result.is_ok());

    let path = Utf8PathBuf::from(if cfg!(target_family = "windows") {
        r"\\?\C:\src\app.gleam"
    } else {
        "/src/app.gleam"
    });

    let url = Url::from_file_path(path).expect("should be valid path for url");

    let params = InlayHintParams {
        text_document: TextDocumentIdentifier::new(url),
        work_done_progress_params: Default::default(),
        range: Range::new(Position::new(0, 0), Position::new(0, 0)),
    };
    let response = engine.inlay_hint(params);

    response.result.expect("inlay hint request should not fail")
}

#[test]
fn module_constants() {
    let code = "
const n = 42
";
    expect_hints(
        code,
        vec![InlayHint {
            position: Position::new(1, 7),
            label: ": Int".to_owned().into(),
            kind: Some(InlayHintKind::TYPE),
            text_edits: Some(vec![TextEdit {
                range: Range::new(Position::new(1, 7), Position::new(1, 7)),
                new_text: ": Int".to_owned(),
            }]),
            tooltip: None,
            padding_left: None,
            padding_right: None,
            data: None,
        }],
    );
}

#[test]
fn function_definitions() {
    let code = "
fn add(lhs, rhs) {
    lhs + rhs
}
";
    expect_hints(
        code,
        vec![
            InlayHint {
                position: Position::new(1, 16),
                label: "-> Int".to_owned().into(),
                kind: Some(InlayHintKind::TYPE),
                text_edits: Some(vec![TextEdit {
                    range: Range::new(Position::new(1, 16), Position::new(1, 16)),
                    new_text: " -> Int".to_owned(),
                }]),
                tooltip: None,
                padding_left: Some(true),
                padding_right: None,
                data: None,
            },
            InlayHint {
                position: Position::new(1, 10),
                label: ": Int".to_owned().into(),
                kind: Some(InlayHintKind::TYPE),
                text_edits: Some(vec![TextEdit {
                    range: Range::new(Position::new(1, 10), Position::new(1, 10)),
                    new_text: ": Int".to_owned(),
                }]),
                tooltip: None,
                padding_left: None,
                padding_right: None,
                data: None,
            },
            InlayHint {
                position: Position::new(1, 15),
                label: ": Int".to_owned().into(),
                kind: Some(InlayHintKind::TYPE),
                text_edits: Some(vec![TextEdit {
                    range: Range::new(Position::new(1, 15), Position::new(1, 15)),
                    new_text: ": Int".to_owned(),
                }]),
                tooltip: None,
                padding_left: None,
                padding_right: None,
                data: None,
            },
        ],
    );
}
