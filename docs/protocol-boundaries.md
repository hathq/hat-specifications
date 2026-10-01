# Protocol boundaries

This is the normative map for the protocol layers used by Hatter and HATs.
It is intentionally small: a feature reuses one row and does not create a new
meaning of “protocol”. Application schemas and Rust validators in this repository govern HAT authority.
sem-lang alone governs language meaning; an application validator must not replace it.

Identifier ownership and the separation between exact URI identity and semantic
hierarchy are defined in [identifier-hierarchy.md](identifier-hierarchy.md).

## The four boundaries

| Boundary | Answers | Owner | Canonical contract |
| --- | --- | --- | --- |
| Semantic meaning | What does this request or result mean? | sem-lang | `sem-lang://wire/semantic-outcome/v1` |
| HAT operation | What bounded operation is requested, completed or failed? | HatSpec | invocation, result and failure contracts |
| Communication capability | Which exact external exchange convention can an implementation speak? | the standards or ecosystem protocol owner; HatSpec owns only the declaration shape | `hathq://hat/communication-capability/v1` |
| Hatter management | Which local owner operation does CLI or Console request? | Hatter | Hatter's closed versioned JSONL management contract |

The Codex-derived Rust crate is not a fifth protocol. It is
`hatter-engine-types`, an internal implementation dependency. A Rust type does
not become an ecosystem contract unless a separately versioned Hatter or
HatSpec schema names it.

## Feature map

| Feature | Meaning and operation boundary | Communication owner | Prohibited duplication |
| --- | --- | --- | --- |
| HAT installation and catalog reading | Hatter management plus exact signed HatSpec package | acquisition source owner | browser upload contract, inferred trust, copied catalog terms |
| HAT invocation and worker result | HatSpec invocation/result/failure | declaring worker or ecosystem adapter | worker-specific operation vocabulary in Hatter |
| Inference preparation | Exact sem-lang outcome plus independent HAT authorization and selected model route | selected model provider adapter | provider request fields in the foundation vocabulary |
| HTTP service use | package operation plus communication capability | IETF reference and concrete adapter | an `http` entity, endpoint or credential in Hatter vocabulary |
| Zixcel or Crowsi service use | package operation plus communication capability | exact Zixcel or Crowsi repository | copied wire schema, route, custody or policy in Hatter |
| Timeline and runtime journal | shared event semantics with distinct persistence owners | none unless an exchange is separately declared | treating storage records as a transport protocol |
| CLI and Console parity | one Hatter management operation | Hatter local runtime | a Console-only business operation |

## Deterministic rules

1. A package says it can speak a protocol only through one exact four-field
   `CommunicationProtocolReference`: owner, opaque protocol identity, positive
   version and immutable specification reference.
2. A service requirement can constrain operations and accepted protocols.
   Resolution requires one installed provider containing every requested
   operation and, when constrained, an exactly equal protocol reference.
3. Labels, URI prefixes, standards families, compatible-looking versions and
   model judgment never establish compatibility.
4. Communication declarations contain no address, credential, account, route,
   placement, liveness, permission or authority. Their respective owner
   services supply those values after normal Hatter authorization.
5. Provider domain meaning belongs to the provider's SemanticCatalog. Only
   `communication protocol`, `communication capability` and `speaks` are shared
   generic terms in the versioned `hathq.communication` catalog.
6. Unknown, missing and ambiguous values remain explicit and fail closed.

## Reading the repository

- `schemas/hat-communication-capability-v1.schema.json` defines the wire shape.
- `src/communication.rs` defines validation and the shared communication
  catalog.
- `src/package_model.rs` attaches declarations and accepted protocols to a HAT
  package.
- `src/package_validation.rs` enforces operation ownership and the exact catalog
  dependency.
- `docs/communication-protocol.md` explains vocabulary and ownership examples.
