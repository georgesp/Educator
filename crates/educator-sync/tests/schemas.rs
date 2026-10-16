//! The exchange format (SUB027606): the schemas and the examples of `schemas/` agree, the messages
//! written by the application follow the schemas, and a message of another version or malformed
//! is refused with a clear error.

use educator_core::sync::ChangeBatch;
use educator_sync::wire::*;
use serde_json::Value;
use std::path::PathBuf;

fn folder() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("schemas")
}

fn read(path: &std::path::Path) -> Value {
    serde_json::from_str(&std::fs::read_to_string(path).unwrap())
        .unwrap_or_else(|e| panic!("{}: {e}", path.display()))
}

/// A validator of one message of the schema (`$defs/<name>`).
fn validator(name: &str) -> jsonschema::Validator {
    let mut schema = read(&folder().join("sync-messages.schema.json"));
    assert!(schema["$defs"][name].is_object(), "no definition {name}");
    schema["$ref"] = Value::String(format!("#/$defs/{name}"));
    jsonschema::draft202012::new(&schema).unwrap()
}

fn errors(name: &str, value: &Value) -> Vec<String> {
    validator(name)
        .iter_errors(value)
        .map(|e| format!("{}: {e}", e.instance_path))
        .collect()
}

/// Example file → definition of the schema.
const EXAMPLES: [(&str, &str); 9] = [
    ("pair-request.json", "pairRequest"),
    ("pair-response.json", "pairResponse"),
    ("hello-request.json", "helloRequest"),
    ("hello-response.json", "helloResponse"),
    ("push-request.json", "pushRequest"),
    ("push-response.json", "pushResponse"),
    ("pull-request.json", "pullRequest"),
    ("pull-response.json", "pullResponse"),
    ("error-response.json", "errorResponse"),
];

#[test]
fn every_example_follows_its_schema() {
    let files = std::fs::read_dir(folder().join("examples"))
        .unwrap()
        .count();
    assert_eq!(
        EXAMPLES.len(),
        files,
        "an example without its line in EXAMPLES"
    );
    for (file, name) in EXAMPLES {
        let value = read(&folder().join("examples").join(file));
        let found = errors(name, &value);
        assert!(found.is_empty(), "{file}:\n{}", found.join("\n"));
    }
}

#[test]
fn the_examples_are_read_by_the_application() {
    let body = std::fs::read(folder().join("examples/push-request.json")).unwrap();
    let push: PushRequest = read_message(&body).unwrap();
    let batch = ChangeBatch::try_from(&push.batch).unwrap();
    assert_eq!(1, batch.sessions.len());
    assert_eq!(2, batch.sessions[0].answers.len());
    assert_eq!(1, batch.reviews.len());

    let body = std::fs::read(folder().join("examples/pull-response.json")).unwrap();
    let pull: PullResponse = read_message(&body).unwrap();
    assert!(pull.more);
    let body = std::fs::read(folder().join("examples/hello-response.json")).unwrap();
    let hello: HelloResponse = read_message(&body).unwrap();
    assert_eq!(1, profile_states(&hello.profiles).unwrap().len());
}

#[test]
fn what_the_application_writes_follows_the_schema_and_round_trips() {
    let body = std::fs::read(folder().join("examples/push-request.json")).unwrap();
    let push: PushRequest = read_message(&body).unwrap();
    let batch = ChangeBatch::try_from(&push.batch).unwrap();

    // Core types → message → JSON: valid, and read back identical.
    let written = PushRequest {
        batch: WireBatch::from(&batch),
        ..push.clone()
    };
    let json = serde_json::to_value(&written).unwrap();
    let found = errors("pushRequest", &json);
    assert!(found.is_empty(), "{}", found.join("\n"));
    let again: PushRequest = read_message(&serde_json::to_vec(&written).unwrap()).unwrap();
    assert_eq!(batch, ChangeBatch::try_from(&again.batch).unwrap());

    let empty = serde_json::to_value(PullResponse {
        protocol: PROTOCOL_VERSION,
        last_seq: 0,
        more: false,
        batch: WireBatch::default(),
    })
    .unwrap();
    assert!(errors("pullResponse", &empty).is_empty());
}

#[test]
fn another_version_or_a_malformed_message_is_refused_without_panic() {
    let pull = |json: &str| read_message::<PullRequest>(json.as_bytes());
    assert_eq!(
        Err(MessageError::Incompatible(2)),
        pull(r#"{"protocol": 2, "deviceId": "0199f1a2-3b4c-7d5e-8f60-718293a4b5c6", "after": 0}"#)
    );
    for json in [
        "",
        "pas du json",
        "[]",
        r#"{"deviceId": "0199f1a2-3b4c-7d5e-8f60-718293a4b5c6", "after": 0}"#,
        r#"{"protocol": 1, "deviceId": "pas un identifiant", "after": 0}"#,
        r#"{"protocol": 1, "deviceId": "0199f1a2-3b4c-7d5e-8f60-718293a4b5c6"}"#,
    ] {
        assert!(
            matches!(pull(json), Err(MessageError::Malformed(_))),
            "{json}"
        );
    }

    // A batch whose values the merge cannot use: refused before it reaches the database.
    let body = std::fs::read_to_string(folder().join("examples/push-request.json")).unwrap();
    for (from, to) in [
        (r#""grade": "CE1""#, r#""grade": "6e""#),
        (r#""type": "Exercise""#, r#""type": "Game""#),
        ("0001792090060000-000000-", "17920-0-"),
        (
            r#""lessonId": "ce1-tables-addition""#,
            r#""lessonId": "a/b""#,
        ),
        (r#""pointsEarned": 30"#, r#""pointsEarned": -30"#),
        (r#""name": "Léa""#, r#""name": "   ""#),
    ] {
        let changed = body.replacen(from, to, 1);
        assert_ne!(body, changed, "{from}");
        let push: PushRequest = read_message(changed.as_bytes()).unwrap();
        assert!(ChangeBatch::try_from(&push.batch).is_err(), "{from} → {to}");
    }
}
