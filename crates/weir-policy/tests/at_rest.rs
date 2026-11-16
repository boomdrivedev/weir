//! Facts at rest are one `fact` field per value (C.2, C.9; record 0010).

use serde_json::json;
use weir_policy::Facts;

#[test]
fn facts_at_rest_in_their_names_order() {
    let stated =
        Facts::stated(&json!({"level": "patch", "attendee": ["b@acme.com", "a@acme.com"]}))
            .expect("stated facts");
    let fields = stated.to_fields();
    assert_eq!(
        fields,
        [
            r#"attendee "b@acme.com""#,
            r#"attendee "a@acme.com""#,
            r#"level "patch""#
        ]
    );
    let read = Facts::from_fields(fields.iter().map(String::as_str)).expect("facts at rest");
    assert_eq!(read, stated);

    // A fact of one value and a list of one read back alike.
    let one = Facts::stated(&json!({"length": 900, "released": "2026-11-14", "recorded": true}))
        .expect("stated facts");
    let listed =
        Facts::stated(&json!({"length": [900], "released": ["2026-11-14"], "recorded": [true]}))
            .expect("stated facts");
    assert_eq!(one.to_fields(), listed.to_fields());
    let fields = one.to_fields();
    assert_eq!(
        fields,
        ["length 900", "recorded true", r#"released "2026-11-14""#]
    );
    assert_eq!(
        Facts::from_fields(fields.iter().map(String::as_str)).expect("facts at rest"),
        listed
    );
}

#[test]
fn a_fact_field_out_of_shape() {
    let error = Facts::from_fields(["level patch"]).expect_err("not JSON");
    assert_eq!(error.field, "fact");
    assert!(error.reason.contains("`level patch`"), "{error}");
    for field in [
        "level",
        r#"level  "patch""#,
        r#"Level "patch""#,
        r#"says "request""#,
        r#"attendee.client true"#,
        r#"title "Daily stand-up""#,
        r#"level {"major": 2}"#,
        r#"level ["patch"]"#,
        "level null",
    ] {
        let error = Facts::from_fields([field]).expect_err(field);
        assert_eq!(error.field, "fact", "{field}");
    }
}
