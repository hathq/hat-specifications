# OSS source contract: hathq/hat-specifications

HAT package identity, capabilities, scope, grants and execution contracts.

## Responsibility boundary

sem-lang owns meaning, canonical IR, compilation outcomes and memory rules. Catalog/clause/exchange linguistic migration remains outstanding until actual code and consumer evidence establish its completion.

Same-function managed packages take priority when their actual capabilities fit. Do not add an unrelated dependency merely to connect organizations. Preserve upstream LICENSE/NOTICE and immutable release/history evidence; unused compatibility sources belong in Git history, not the supported API.

## Package ownership

| Manifest | Package | Distribution policy |
| --- | --- | --- |
| `Cargo.toml` | `hat-specifications` | declared registry candidate; verify actual registry artifact |
| `crates/hat-specifications-cli/Cargo.toml` | `hat-specifications-cli` | source-only; publish=false |

Manifest versions and publish flags are declarations, not proof of published availability. Public npm exports and Rust implementation/docs belong to the manifests above; generated dist and sibling checkouts are not source-of-truth implementations.

## Source completion criteria

1. The documented purpose, public exports, validators/schema and actual implementation agree. Remove only confirmed-unused aliases, schemas and compatibility adapters after checking consumers; a shared field name does not imply a shared trust responsibility.
2. Runtime and official dependency resolution work from an independent checkout with committed locks. No local sibling artifact, private registry or ambient owner credential is required for the OSS source path.
3. Supported behavior has reproducible examples/playground and functional tests, including invalid input and relevant failure/cancellation/replay/tenant boundaries. A policy-only CI or planning fixture does not prove external effects.
4. CI runs the declared functional acceptance without disguising skipped/unavailable checks as success. Changed public package bytes receive a new reviewed release; immutable old versions are never overwritten.
5. License/NOTICE, security reporting, usage and the actual supported/unsupported scope stay in the owning repository. The owner operates independently of a central private management checkout.
6. Source acceptance, artifact/registry acceptance and production deployment are separate records. This document defines the target; it does not declare unexecuted tests, unreleased APIs or live-account validation completed.

## Existing verification entrypoints

- [.github/workflows/ci.yml](../.github/workflows/ci.yml)
- [.github/workflows/oss-policy.yml](../.github/workflows/oss-policy.yml)

Use the package scripts/owner CI above and the repository README for exact commands. When schema files exist, their canonical validator and consumer must be checked together; deleting a definition on a text search alone is insufficient.

## Acceptance record

Inventory baseline source: `5812bdf839dcdf9c82857f68c81548116f2db249`. This baseline records observed files and declared interfaces, not a completed audit. Subsequent source changes are accepted at their exact commit with test evidence. Production hosting, account login, secret custody installation and data migration remain outside source-only acceptance.

## Owned source and acceptance map

[oss-source-map.json](oss-source-map.json) is the tracked map of package
interfaces, declared commands, schemas, functional test sources, examples and CI
jobs. It keeps each responsibility attached to its actual physical source owner.
Public exports are supported only within this repository's responsibility boundary;
a filename or schema declaration does not grant product or cloud authority.

| Package | Canonical manifest | Responsibility/interface |
| --- | --- | --- |
| `hat-specifications` | [Cargo.toml](../Cargo.toml) | Versioned HAT and Fitting contract validator |
| `hat-specifications-cli` | [crates/hat-specifications-cli/Cargo.toml](../crates/hat-specifications-cli/Cargo.toml) | Standalone read-only HAT schema and fitting validator |

For source acceptance, run each applicable functional CI job with the committed
locks from an independent checkout. Match public entrypoints to their test/example
sources in the map; test the relevant invalid input and failure boundary as well as
the normal path. Policy-only, skipped and credential-dependent jobs do not establish
functional coverage. Release jobs establish artifact acceptance only when actually
run at the reviewed commit. A source-only/example repository must not be presented
as a complete external service or a released package.

Schema definitions in the map remain owned by their canonical validator and
consumers. An absent schema file means no separate schema file is declared; it does
not mean the runtime interface has no validation. Do not introduce another common
schema solely to mirror an implementation type.

## Retired exchange prototype

The unused SemanticExchange/comprehension report API, identity-match classifier,
digest and two schema documents are removed from this source candidate. An exact
catalog ID/digest match is not proof that a language owner understood a statement.
Managed-source references were limited to the prototype, its doctor registration
and dedicated tests; there is no replacement alias or compatibility decoder.
Published artifacts remain immutable; the package version remains 0.10.0 as
instructed. A future release must review this source API removal.

Operational catalog/clause metadata still has real package/product consumers and
is not deleted on a text match. Its remaining language/operation responsibility
boundary must be established with those consumers. Meaning, canonical IR and
compilation outcomes continue to belong to sem-lang.

## Unused standalone clause and presentation retirement

The standalone SemanticClause/modality/role-binding wire model, clause validator,
clause schema and locale clause renderer were used only by their own prototype
fixtures. Their exports, sources, schema and doctor entry are retired without aliases
or a replacement language protocol. sem-lang's independent linguistic definitions
are unaffected. Operational catalog references remain; the later catalog-frame
retirement below changes the development source shape and its digests. Source
version remains 0.10.0, with existing published artifacts unchanged.

## Operational declaration without linguistic-frame inference

Operation validation requires one package-owned action bound to its exact input wire schema. It does not additionally require a linguistic clause frame. Frame absence does not relax capability, action, grant, scope, input/output schema or package identity requirements. Language meaning belongs to Sem.

The development catalog no longer contains clause frames or their role/digest/validator APIs. Old JSON with a `frames` field is rejected, including an empty array; no alias or compatibility decoder is added. Language meaning still belongs to sem-lang. Display lexicalizations and operational term references remain a separate responsibility audit, not a completed semantic migration.

Removing even empty frame metadata changes canonical catalog bytes, dependencies, package hashes and outer signed selection. External producers and consumers must be updated and tested together before artifact release. Existing immutable releases must not be rehashed in place. See [catalog source shape](catalog-source-shape.md) for acceptance limits.
