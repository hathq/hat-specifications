# Semantic ownership and producer acceptance

This inventory belongs to issue #23. Source version stays **0.10.0**. It does not
replace a registry crate, signed package/index or installed artifact.

## Responsibility inventory

| Current source | Retained responsibility | Boundary and remaining work |
| --- | --- | --- |
| `src/package_model.rs`, `package_validation.rs`, `operations.rs` | Package identity, wire-bound actions, declared capabilities, context and handler/reducer contracts | Declaring an operation neither compiles language nor authorizes dispatch |
| `src/semantic_catalog.rs`, `semantic_catalog_validation.rs`, `semantic_catalog_digest.rs` | Exact catalog/term references, kinds, wire schemas, dependencies and digest consistency | `is_a`, `domain`, `range` are checked references; this validator does not evaluate linguistic meaning or adopt sem-lang definitions |
| `src/foundation.rs`, `communication.rs` | Current foundation and protocol references required by package consumers | Existing `core.*`/`world.*` vocabulary remains migration debt. Do not rename/delete it independently of real consumers or claim it is sem-lang's canonical vocabulary |
| `SemanticLexicalization` in the catalog | Explicit locale, preferred display text and search aliases attached to exact terms | Display metadata is not a parser, resolver, disambiguation rule or authenticated identity. Language lexical/semantic interpretation belongs to sem-lang |
| `src/semantic_model.rs`, `semantic_validation.rs` | Existing reference-bound application contribution envelopes and shape checks | Entity/relation/assertion/event DTOs still have consumers. They are not a second canonical IR, verified truth or sem-lang adoption result; migration is not complete |
| `src/runtime_model.rs`, `runtime_validation.rs` | Invocation/package/operation/input-reference contract consistency | Shape validation does not verify a real grant issuer, expiry, revocation, subject/scope policy, installed bytes or provider permission |
| `src/control_grant_validation.rs`, `operational_validation.rs` | Explicit declarative grant and operational lifecycle contracts | Runtime owners must verify actual authority and current state before dispatch; package validation cannot supply credentials or authority |
| `src/inference_action.rs`, `perception.rs`, `information_axis.rs` | Inspectable product proposals, source/evidence references and routing declarations | A selected proposal or Complete state is neither language adoption nor an execution grant |
| Retired clause, exchange and frame prototypes | No current source responsibility | No compatibility decoder, alias, replacement language schema or read-time migration is restored |

The generic source decomposer remains in Zixcel, the source input binding in
`hathq/hat-source-curator`, and structural/language compilation in sem-lang. No
sem-lang implementation is copied into this crate, and no HAT dependency is added
to a foundational language crate.

## Independent public producer cohort

`tests/fixtures/public-producers/sources.json` selects exact original descriptor
bytes from public source commits, including Git blob identity, SHA-256, package
version and catalog digest. The two selected producers are Source Curator and
Website Operator, both HAT **0.10.1**. The original JSON is preserved byte for byte,
including its source license and attribution; no test-only catalog is substituted.

The ordinary workspace test command exercises:

- `tests/public_producer_cohort.rs`: exact raw hashes, source package/version,
  recomputed catalog digests and the current foundation dependency; rejection of
  modified wire contracts, catalog/term identities, old empty frames and unsupported
  package schemas; invocation binding to each real package's action/input schema.
- `crates/hat-specifications-cli/tests/cli.rs`: the separate CLI process accepts
  both descriptors deterministically and reports `external_actions: false`.

The same source check runs locally or in CI:

```sh
cargo +1.97 test --locked --workspace --all-targets --all-features
```

No network, producer process, model, provider login or host installation is needed
to run these offline acceptance cases. Fixture provenance is selected at review
time; the tests verify those selected bytes, not whatever a mutable branch serves
later. Hash consistency is not publisher trust. A source descriptor is not proof
of an anonymously downloadable release or a signed Registry selection.

## Acceptance still outstanding

The current cohort establishes package/CLI/invocation **source contract** acceptance.
It does not establish socket/RPC execution, authenticated subject/scope, a current
installed grant, live provider permission, real inference or product E2E behavior.
An opaque but well-formed grant reference can pass a shape validator and must
still be resolved and authorized by the execution owner.

Catalog display/reference inventory is now explicit; actual migration of remaining
foundation/application meaning requires coordinated sem-lang definitions and real
product consumers. The sem-lang producer-composition work in
`semantic-language/sem-lang#1` is separate from accepted model execution and is not
a replacement for that migration. Keep issue #23 open until these consumer and
ownership gaps are resolved. A later public API removal requires an explicitly
chosen version and coordinated caller updates. Never rewrite released bytes.
