use crate::model::validation;
use crate::tokens::{lower_hex_32, schema_uri, semantic_version, stable_token};
use crate::{
    ACTION_RESULT_SCHEMA, ACTION_STATUS_SCHEMA, ActionReference, BINDING_SCHEMA, CHECKPOINT_SCHEMA,
    HatActionResult, HatActionStatus, HatBinding, HatCheckpoint, HatInvocation,
    HatInvocationControl, HatInvocationOutcome, HatPackage, HatProjection, HatProjectionEvent,
    HatProjectionJournal, INVOCATION_CONTROL_SCHEMA, INVOCATION_SCHEMA, PROJECTION_EVENT_SCHEMA,
    PROJECTION_JOURNAL_SCHEMA, PROJECTION_SCHEMA, Validation,
};

#[must_use]
pub fn validate_binding(binding: &HatBinding) -> Validation {
    let mut findings = Vec::new();
    if binding.schema != BINDING_SCHEMA || !binding.package_id.starts_with("hat/") {
        findings.push("binding identity is invalid".to_owned());
    }
    for digest in [
        &binding.package_digest_sha256,
        &binding.catalog_digest_sha256,
        &binding.fitting_digest_sha256,
    ] {
        if !lower_hex_32(digest) {
            findings.push("binding digest is invalid".to_owned());
        }
    }
    if binding.subject_ref.is_empty() || binding.subject_ref.len() > 256 {
        findings.push("subject_ref is invalid".to_owned());
    }
    if binding.scope_ref.is_empty() || binding.scope_ref.len() > 256 {
        findings.push("scope_ref is invalid".to_owned());
    }
    validation(findings)
}

#[must_use]
pub fn validate_invocation(invocation: &HatInvocation) -> Validation {
    let mut findings = validate_binding(&invocation.binding).findings;
    if invocation.schema != INVOCATION_SCHEMA
        || !stable_token(&invocation.invocation_id)
        || !schema_uri(&invocation.operation_id)
        || !stable_token(&invocation.idempotency_key)
    {
        findings.push("invocation identity is invalid".to_owned());
    }
    if !stable_token(&invocation.context_partition.context_partition_id)
        || !lower_hex_32(&invocation.context_partition.policy_digest_sha256)
    {
        findings.push("context partition is invalid".to_owned());
    }
    for reference in [
        &invocation.input,
        &invocation.effective_grant,
        &invocation.placement,
    ] {
        validate_action_reference(reference, &mut findings);
    }
    validation(findings)
}

#[must_use]
pub fn validate_invocation_against_package(
    invocation: &HatInvocation,
    package: &HatPackage,
) -> Validation {
    let mut findings = validate_invocation(invocation).findings;
    if invocation.binding.package_id != package.package_id {
        findings.push("invocation package binding is invalid".to_owned());
    }
    match package
        .operations
        .iter()
        .find(|operation| operation.id == invocation.operation_id)
    {
        Some(operation) if operation.input_schema == invocation.input.schema_id => {}
        Some(_) => findings.push("invocation input schema is invalid".to_owned()),
        None => findings.push("invocation operation is not installed".to_owned()),
    }
    validation(findings)
}

#[must_use]
pub fn validate_action_status(status: &HatActionStatus) -> Validation {
    let mut findings = Vec::new();
    if status.schema != ACTION_STATUS_SCHEMA
        || !stable_token(&status.invocation_id)
        || !stable_token(&status.context_partition_id)
    {
        findings.push("action status identity is invalid".to_owned());
    }
    let terminal = matches!(
        status.phase,
        crate::HatInvocationPhase::Failed | crate::HatInvocationPhase::Cancelled
    );
    if terminal != status.reason_id.is_some()
        || status.reason_id.as_ref().is_some_and(|id| !schema_uri(id))
    {
        findings.push("action status reason is invalid".to_owned());
    }
    validation(findings)
}

#[must_use]
pub fn validate_action_result(result: &HatActionResult) -> Validation {
    let mut findings = Vec::new();
    if result.schema != ACTION_RESULT_SCHEMA
        || !stable_token(&result.invocation_id)
        || !schema_uri(&result.operation_id)
    {
        findings.push("action result identity is invalid".to_owned());
    }
    match result.outcome {
        HatInvocationOutcome::Completed => {
            if result.output.is_none() || result.reason_id.is_some() {
                findings.push("completed action result is invalid".to_owned());
            }
        }
        HatInvocationOutcome::Failed
        | HatInvocationOutcome::Denied
        | HatInvocationOutcome::Cancelled => {
            if result.output.is_some() || result.reason_id.as_ref().is_none_or(|id| !schema_uri(id))
            {
                findings.push("non-completed action result is invalid".to_owned());
            }
        }
    }
    if let Some(output) = &result.output {
        validate_action_reference(output, &mut findings);
    }
    validate_evidence(&result.evidence_refs, &mut findings);
    validation(findings)
}

#[must_use]
pub fn validate_invocation_control(control: &HatInvocationControl) -> Validation {
    let mut findings = Vec::new();
    if control.schema != INVOCATION_CONTROL_SCHEMA
        || !stable_token(&control.request_id)
        || !stable_token(&control.invocation_id)
        || !stable_token(&control.context_partition_id)
        || !stable_token(&control.idempotency_key)
    {
        findings.push("invocation control identity is invalid".to_owned());
    }
    validation(findings)
}

fn validate_action_reference(reference: &ActionReference, findings: &mut Vec<String>) {
    if !stable_token(&reference.owner_id)
        || reference.reference.is_empty()
        || reference.reference.len() > 512
        || !schema_uri(&reference.schema_id)
        || !lower_hex_32(&reference.digest_sha256)
    {
        findings.push("action reference is invalid".to_owned());
    }
}

#[must_use]
pub fn validate_projection_event(event: &HatProjectionEvent) -> Validation {
    let mut findings = Vec::new();
    if event.schema != PROJECTION_EVENT_SCHEMA
        || !stable_token(&event.event_id)
        || !stable_token(&event.invocation_id)
        || !stable_token(&event.context_partition_id)
        || !schema_uri(&event.operation_id)
        || event.next_revision != event.previous_revision.saturating_add(1)
    {
        findings.push("event identity or revision is invalid".to_owned());
    }
    if crate::semantic_validation::validate_event_semantics(&event.semantics).is_err()
        || event.semantics.event_type != "core.event.action.completed"
        || event.semantics.correlation_id.as_deref() != Some(event.invocation_id.as_str())
    {
        findings.push("event semantics are invalid".to_owned());
    }
    validate_evidence(&event.evidence_refs, &mut findings);
    validation(findings)
}

#[must_use]
pub fn validate_projection_journal(timeline: &HatProjectionJournal) -> Validation {
    let mut findings = Vec::new();
    if timeline.schema != PROJECTION_JOURNAL_SCHEMA
        || !stable_token(&timeline.context_partition_id)
        || !timeline.package_id.starts_with("hat/")
        || timeline.events.len() > 256
    {
        findings.push("timeline identity or size is invalid".to_owned());
    }
    let mut expected = timeline.from_revision;
    for event in &timeline.events {
        findings.extend(validate_projection_event(event).findings);
        if event.context_partition_id != timeline.context_partition_id
            || event.previous_revision != expected
        {
            findings.push("timeline events are not contiguous".to_owned());
        }
        expected = event.next_revision;
    }
    if expected != timeline.to_revision {
        findings.push("timeline terminal revision is invalid".to_owned());
    }
    validation(findings)
}

#[must_use]
pub fn validate_projection(projection: &HatProjection) -> Validation {
    let mut findings = Vec::new();
    if projection.schema != PROJECTION_SCHEMA
        || !stable_token(&projection.context_partition_id)
        || !projection.package_id.starts_with("hat/")
        || projection.timeline_watermark < projection.revision
        || !lower_hex_32(&projection.projection_digest_sha256)
    {
        findings.push("projection identity or revision is invalid".to_owned());
    }
    validate_evidence(&projection.evidence_refs, &mut findings);
    for unresolved in &projection.unresolved_terms {
        if !lower_hex_32(&unresolved.input_digest_sha256)
            || !lower_hex_32(&unresolved.catalog_digest_sha256)
            || unresolved
                .resolution_hat_id
                .as_ref()
                .is_some_and(|id| !id.starts_with("hat/"))
        {
            findings.push("unresolved term is invalid".to_owned());
        }
    }
    validation(findings)
}

#[must_use]
pub fn validate_checkpoint(checkpoint: &HatCheckpoint) -> Validation {
    let mut findings = Vec::new();
    if checkpoint.schema != CHECKPOINT_SCHEMA
        || !stable_token(&checkpoint.context_partition_id)
        || !checkpoint.package_id.starts_with("hat/")
        || checkpoint.to_watermark < checkpoint.from_watermark
        || !lower_hex_32(&checkpoint.previous_projection_digest_sha256)
        || !lower_hex_32(&checkpoint.new_projection_digest_sha256)
        || !semantic_version(&checkpoint.reducer_version)
    {
        findings.push("checkpoint is invalid".to_owned());
    }
    validation(findings)
}

fn validate_evidence(evidence: &[crate::EvidenceReference], findings: &mut Vec<String>) {
    if evidence.len() > 256 {
        findings.push("too many evidence references".to_owned());
    }
    for item in evidence {
        if !stable_token(&item.owner_id)
            || item.reference.is_empty()
            || item.reference.len() > 512
            || !lower_hex_32(&item.digest_sha256)
        {
            findings.push("evidence reference is invalid".to_owned());
        }
    }
}
