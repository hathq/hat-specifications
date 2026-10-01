use hat_specifications::{
    SITUATION_CONTRIBUTION_SCHEMA, SituationContribution, SituationKind, SituationSource,
    SituationStatus, SituationTime, validate_situation_contribution,
};

fn contribution() -> SituationContribution {
    SituationContribution {
        schema: SITUATION_CONTRIBUTION_SCHEMA.to_owned(),
        contribution_id: "review-accounts".to_owned(),
        category_term_id: "hathq://vocabulary/data-domain/finance/v1".to_owned(),
        kind: SituationKind::Task,
        title: "会計を確認".to_owned(),
        status: SituationStatus::Planned,
        time: Some(SituationTime {
            start_at_unix_ms: 1,
            end_at_unix_ms: None,
            time_zone: "Asia/Tokyo".to_owned(),
        }),
        place: None,
        actors: vec!["わたし".to_owned()],
        object_label: Some("会計".to_owned()),
        detail_handle: "review-accounts".to_owned(),
        source: SituationSource {
            repository_id: "hat-accounting".to_owned(),
            digest_sha256: "a".repeat(64),
            revision: 1,
        },
    }
}

#[test]
fn validates_one_common_event_or_action_contribution() {
    assert!(validate_situation_contribution(&contribution()).valid);
    let mut invalid = contribution();
    invalid.source.revision = 0;
    assert!(!validate_situation_contribution(&invalid).valid);
}

#[test]
fn closed_wire_does_not_accept_an_independent_todo_body() {
    let mut value = serde_json::to_value(contribution()).expect("serialize");
    value
        .as_object_mut()
        .expect("object")
        .insert("todo".to_owned(), serde_json::json!({ "done": false }));
    assert!(serde_json::from_value::<SituationContribution>(value).is_err());
}
