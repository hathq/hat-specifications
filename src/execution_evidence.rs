//! Provider-owned historical results. These are execution contracts, not meaning.
use crate::{
    ActionReference, HatActionResult, HatFederationExecutionReceipt, HatInvocation, Validation,
};
use serde::{Deserialize, Serialize};
use zixcel_revision::attestation::{Attestation, VerificationMaterial, valid_digest};

/// Only implemented recovery mechanisms may be advertised. Other providers stay
/// explicitly unrecoverable; lack of evidence never grants retry permission.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case", deny_unknown_fields)]
pub enum HatEvidenceRecovery {
    Unrecoverable {},
    DurableEvidence {
        authority: VerificationMaterial,
        /// Logical, read-only transport route; no host address or execute command.
        query_route_ref: String,
    },
}
impl HatEvidenceRecovery {
    #[must_use]
    pub fn valid_declaration(&self) -> bool {
        match self {
            Self::Unrecoverable {} => true,
            Self::DurableEvidence {
                authority,
                query_route_ref,
            } => {
                authority.validate_declaration().is_ok()
                    && query_route_ref.starts_with("crowsi/")
                    && query_route_ref.len() <= 256
                    && query_route_ref.bytes().all(|b| b.is_ascii_graphic())
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HatExecutionEvidenceBinding {
    /// Digest of the complete original immutable admission, not current state.
    pub execution_ref: ActionReference,
    pub invocation: HatInvocation,
    pub provider_ref: ActionReference,
    pub provider_generation: String,
    pub worker_ref: ActionReference,
    pub worker_generation: String,
    pub capability_ref: ActionReference,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HatExecutionEvidenceStatement {
    pub binding: HatExecutionEvidenceBinding,
    /// The existing result schema; referenced output bytes remain provider-owned.
    pub result: HatActionResult,
    pub failure: Option<crate::HatFailureEnvelope>,
    pub provider_receipt: Option<HatFederationExecutionReceipt>,
    pub produced_at_epoch_s: u64,
    /// Optional owner-selected expiry is covered by the signature, not a UI TTL.
    pub expires_at_epoch_s: Option<u64>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HatExecutionEvidence {
    pub statement: HatExecutionEvidenceStatement,
    pub attestation: Attestation,
}

#[must_use]
pub fn validate_execution_evidence_binding(binding: &HatExecutionEvidenceBinding) -> Validation {
    let mut findings = crate::validate_invocation(&binding.invocation).findings;
    for reference in [
        &binding.execution_ref,
        &binding.provider_ref,
        &binding.worker_ref,
        &binding.capability_ref,
    ] {
        // Control-owner schema identities are opaque here. HAT operation schema
        // restrictions do not apply to another owner's exact admission identity.
        if [
            &reference.owner_id,
            &reference.reference,
            &reference.schema_id,
        ]
        .iter()
        .any(|v| v.is_empty() || v.len() > 512 || !v.bytes().all(|b| b.is_ascii_graphic()))
            || !valid_digest(&reference.digest_sha256)
        {
            findings.push("execution evidence control reference is invalid".into());
        }
    }
    if [&binding.provider_generation, &binding.worker_generation]
        .iter()
        .any(|s| s.is_empty() || s.len() > 128 || !s.bytes().all(|b| b.is_ascii_graphic()))
    {
        findings.push("execution evidence generation is invalid".into());
    }
    Validation {
        valid: findings.is_empty(),
        findings,
    }
}

#[must_use]
pub fn validate_execution_evidence_statement(value: &HatExecutionEvidenceStatement) -> Validation {
    let binding = &value.binding;
    let mut findings = validate_execution_evidence_binding(binding).findings;
    findings.extend(crate::validate_action_result(&value.result).findings);
    match (&value.result.outcome, &value.failure) {
        (crate::HatInvocationOutcome::Completed, None) => {}
        (crate::HatInvocationOutcome::Completed, Some(_)) => {
            findings.push("completed evidence has a failure".into());
        }
        (_, Some(failure)) => {
            findings.extend(crate::validate_action_failure(&value.result, failure).findings);
        }
        (_, None) => findings.push("non-completed evidence requires its original failure".into()),
    }
    if value.result.invocation_id != binding.invocation.invocation_id
        || value.result.operation_id != binding.invocation.operation_id
        || value.produced_at_epoch_s == 0
        || value
            .expires_at_epoch_s
            .is_some_and(|expiry| expiry <= value.produced_at_epoch_s)
    {
        findings.push("execution evidence identity or signed lifetime is invalid".into());
    }
    if let Some(receipt) = &value.provider_receipt {
        findings
            .extend(crate::validate_federation_execution_receipt(receipt, &value.result).findings);
        if receipt.invocation_id != value.result.invocation_id
            || crate::invocation_digest(&binding.invocation).ok().as_ref()
                != Some(&receipt.invocation_digest_sha256)
            || crate::action_result_digest(&value.result).ok().as_ref()
                != Some(&receipt.result_digest_sha256)
        {
            findings.push("execution evidence differs from provider receipt".into());
        }
    }
    Validation {
        valid: findings.is_empty(),
        findings,
    }
}
