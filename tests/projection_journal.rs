use hat_specifications::{
    EventSemantics, EvidenceReference, HatProjectionEvent, HatProjectionJournal,
    PROJECTION_EVENT_SCHEMA, PROJECTION_JOURNAL_SCHEMA, SemanticTimeInterval,
    validate_projection_journal,
};

const DIGEST: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";

fn event(id: &str, previous_revision: u64) -> HatProjectionEvent {
    let occurred_at = 1_700_000_000_000
        + i64::try_from(previous_revision).expect("test revision fits semantic time");
    HatProjectionEvent {
        schema: PROJECTION_EVENT_SCHEMA.to_owned(),
        event_id: id.to_owned(),
        semantics: EventSemantics {
            event_type: "core.event.action.completed".to_owned(),
            occurred_time: SemanticTimeInterval {
                start_epoch_ms: occurred_at,
                end_epoch_ms: None,
            },
            observed_at_epoch_ms: occurred_at,
            correlation_id: Some("invoke-1".to_owned()),
            causation_id: None,
        },
        invocation_id: "invoke-1".to_owned(),
        context_partition_id: "context-partition-1".to_owned(),
        operation_id: "hathq://vocabulary/action/review-source/v1".to_owned(),
        previous_revision,
        next_revision: previous_revision + 1,
        evidence_refs: vec![EvidenceReference {
            owner_id: "zixcel".to_owned(),
            reference: format!("evidence-{id}"),
            digest_sha256: DIGEST.to_owned(),
        }],
    }
}

#[test]
fn projection_journal_is_bounded_contiguous_and_context_partitioned() {
    let projection_journal = HatProjectionJournal {
        schema: PROJECTION_JOURNAL_SCHEMA.to_owned(),
        context_partition_id: "context-partition-1".to_owned(),
        package_id: "hat/source-curator".to_owned(),
        from_revision: 4,
        to_revision: 6,
        events: vec![event("event-1", 4), event("event-2", 5)],
    };
    assert!(validate_projection_journal(&projection_journal).valid);

    let mut gap = projection_journal.clone();
    gap.events[1].previous_revision = 7;
    gap.events[1].next_revision = 8;
    assert!(!validate_projection_journal(&gap).valid);

    let mut cross_partition = projection_journal;
    cross_partition.events[1].context_partition_id = "context-partition-2".to_owned();
    assert!(!validate_projection_journal(&cross_partition).valid);

    let empty = HatProjectionJournal {
        schema: PROJECTION_JOURNAL_SCHEMA.to_owned(),
        context_partition_id: "context-partition-1".to_owned(),
        package_id: "hat/source-curator".to_owned(),
        from_revision: 6,
        to_revision: 6,
        events: vec![],
    };
    assert!(validate_projection_journal(&empty).valid);

    let mut invalid_empty = empty;
    invalid_empty.to_revision = 7;
    assert!(!validate_projection_journal(&invalid_empty).valid);
}

#[test]
fn projection_journal_has_no_raw_body_escape_hatch() {
    let projection_journal = HatProjectionJournal {
        schema: PROJECTION_JOURNAL_SCHEMA.to_owned(),
        context_partition_id: "context-partition-1".to_owned(),
        package_id: "hat/source-curator".to_owned(),
        from_revision: 4,
        to_revision: 5,
        events: vec![event("event-1", 4)],
    };
    let mut value = serde_json::to_value(projection_journal).expect("serialize");
    value
        .as_object_mut()
        .expect("object")
        .insert("transcript".to_owned(), serde_json::json!(["secret"]));
    assert!(serde_json::from_value::<HatProjectionJournal>(value).is_err());
}
