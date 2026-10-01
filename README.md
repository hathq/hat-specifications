# HAT Specifications

HATHQ's transport-neutral definition of a HAT and its Fitting check. A HAT
describes a role, capabilities, observation/proposal permissions, explicitly
bounded execution permissions, decision boundaries, and information
classifications. It never contains a credential or grants execution by itself.

## Current language boundary

sem-lang owns canonical meaning, compilation outcomes and language memory.
HatSpec owns installed application capabilities, grants, scopes and execution
bindings. A sem-lang Complete result is not permission to execute an operation.
The former HatSpec inference packet, validator and schema have been removed;
there is no compatibility decoder or conversion adapter for that packet.
Remaining catalog/clause/exchange types are application migration debt, not a
replacement language specification. Do not extend their linguistic behavior.
See Hatter's `docs/internal/sem-lang-migration-gate.md` for the cross-repository
migration inventory. Source removal does not replace an already released archive.

## Ownership boundary

This repository owns the canonical interface used to author, package,
validate, install and invoke HATs. HAT implementations do not live here or in
Hatter. Each HAT is maintained in a separate HATHQ repository and releases an
immutable signed package conforming to these schemas.

The current source candidate includes closed natural-person/legal-person
subject references, revisioned positions, represented-capacity grants,
independent information coordinates, evidence-linked assertions, selectable
retention/deletion records, package identity, Vocabulary, context plans and
declarative semantic contributions. Persistent applicability binds a HAT to a
Subject and Scope. Invocation, event, projection and checkpoint envelopes use a
derived `ContextPartition` only for runtime isolation and recovery; it is not a
user workspace or resource identity. The contracts also define
transport-neutral execution locations, bounded location sets, owner-approved
placement selections and correlated federation receipts. Network addresses and
discovery documents remain outside HAT contracts. Hatter consumes a released
version for package verification, local installation and runtime binding; it
must not copy the types or use a local path dependency.

Owner-bound representative roles are five immutable functional archetypes, not
additional persons: decision stewardship (`white-queen`), bounded action
leadership (`white-rabbit`), authorized external exchange (`white-knight`),
independent observation and verification (`cheshire-cat`), and human values and
burden (`alice`). Mutable display names and bounded child roles are instances of
those archetypes. Matters collect typed, reference-only contributions from the
instances, while automation policies advance only one owner-approved delegation
stage at a time. These contracts do not store chat threads, prompt transcripts,
credentials, provider payloads, or executable HAT logic.

Communication uses one capability shape: a speaker declares that it can speak
an exact, versioned protocol owned by a standards body or ecosystem repository.
HTTP, a Zixcel wire contract and a Hatter management protocol therefore use the same
`CommunicationProtocolReference`; none becomes a Hatter-specific entity type.
The declaration binds a protocol to exact package operations, role and
direction, but carries no endpoint, credential, route, permission or transport
policy. See [Communication protocol and vocabulary](docs/communication-protocol.md).
The complete ownership and layer map is in
[Protocol boundaries](docs/protocol-boundaries.md).

Failures use `hathq://hat/failure-envelope/v1`. A HAT supplies an exact reason
term, closed class, recovery mode, responsible party, optional next action,
bounded display parameters and evidence references. It never supplies a raw
provider message, stack, path, command, credential or source payload. Hatter
and agents may therefore retain and present the same semantic failure locally
without interpreting implementation-specific error text.

For each failed invocation, a HAT developer must:

1. select an accepted `reason_id` and one class (`input`, `precondition`,
   `authority`, `dependency`, `availability`, `conflict`, `limit`, `timeout`,
   `cancelled`, or `internal`);
2. state whether recovery is impossible, an owner action, a mechanical retry,
   or an external change, and name the responsible boundary;
3. pass only declared, bounded display parameters and reference-only evidence;
4. emit a terminal action result with the same invocation, operation, state
   revision, and reason; and
5. call `validate_action_failure_with_catalog` with the exact packaged semantic catalog before
   returning the pair.

An owner action requires an exact catalog action. Retry and external-change
failures do not invent an owner TODO. Unknown reasons, extra fields, mismatched
correlation, and implementation error text fail closed.

This repository must not contain HAT domain logic, registry entries,
installation state, provider credentials, Hatter thread state or executable
provider adapters.

```bash
cargo run --offline -- doctor
cargo run --offline -- validate examples/source-curator.hat.toml
cargo run --offline -- fitting \
  examples/source-curator.hat.toml \
  examples/editor.profile.toml
```

Every command emits one `hathq://hat-specifications/result/v1` JSON document.
Released schemas define the interoperability boundary for consumers.

A timeline page may be empty only when its `from_revision` and `to_revision`
are identical. This represents a valid initial or terminal cursor; it never
authorizes fabrication of events or a revision advance.

`doctor` parses the shipped JSON schemas and validates the embedded example
and Fitting result. Validation is fail-closed: classifications are an enum,
IDs have exactly one namespace separator, profiles are validated before
Fitting, and permission operations come from a mode-specific allowlist.
Credential-shaped keys and high-confidence raw secret values are rejected.
The CLI performs no network request or external action.

Library consumers can call `parse_manifest`, `validate_manifest`,
`parse_profile`, and `fit` independently. Documents are closed TOML, bounded
to 1 MiB and 32 nesting levels. Collections also have explicit limits, and
operation permissions remain mode-specific allowlists. An `execute` permission
is only the signed package maximum: Hatter must still issue a narrower runtime
grant and the provider must enforce that exact grant before every effect. The package remains
private until the complete wire set is reviewed and released; source-candidate
schemas are not a registry compatibility claim.

## Repository quality gate

The complete standalone gate is local and starts no service:

```bash
# WONDERLAND_ROOT is the workspace checkout root.
"$WONDERLAND_ROOT/bin/verify-repositories" --rust --tier standard
```
