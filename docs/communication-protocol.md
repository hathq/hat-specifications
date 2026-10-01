# Communication protocol and vocabulary

This document is the normative overview for protocol meaning in the HAT
ecosystem. The machine contract is
`hathq://hat/communication-capability/v1`; its JSON Schema and Rust validator
live in this repository.

## One grammatical relation

Communication is represented as:

```text
speaker -- core.relation.speaks --> exact communication protocol
```

The versioned `hathq.communication` catalog owns only three generic meanings;
the immutable foundation v1 catalog remains unchanged:

- `core.communication-protocol`: a versioned convention used to exchange data;
- `core.communication-capability`: the bounded ability to use that convention;
- `core.relation.speaks`: the relation rendered as “speaks protocol” or
  “規約を話せる”.

HTTP, MCP, a Zixcel provider contract, a Crowsi transport profile and a Hatter
engine session are not new foundation terms. Their owner publishes an exact
`CommunicationProtocolReference`, and a HAT package may reference it from a
`HatCommunicationCapability`. Provider and standards vocabularies therefore do
not accumulate in Hatter.

## Exact reference

A protocol reference contains only:

| Field | Meaning |
| --- | --- |
| `owner_id` | repository or standards owner of the protocol definition |
| `protocol_id` | opaque absolute identity; never parsed for routing or hierarchy |
| `version` | positive exact revision |
| `specification_ref` | immutable specification identity, not a discovery URL |

The protocol-speaking declaration additionally supplies a package-local ID,
client/server/peer role, send/receive/bidirectional direction and the exact HAT
operations that use the protocol. It deliberately excludes addresses,
credentials, account handles, policy, placement, liveness and authority.

## Protocol layers

Meaning, operations and transport are separate boundaries:

1. **Language data** (`sem-lang://wire/semantic-outcome/v1`) carries a sem-lang
   compilation outcome. This is a data schema, not a HAT transport or permission.
2. **HAT operation protocol** (`hathq://hat/invocation/v4` and its correlated
   result/failure contracts) says what exact operation is requested.
3. **Communication protocol capability** says how an implementation can exchange
   those bytes. HTTP and Zixcel/Crowsi protocols belong here.

An operation ID is not a transport. A transport does not imply an operation.
Both must be independently exact when a service requirement constrains both.
Hatter resolves operations and projects protocol compatibility; the external
binding owner resolves address, route, custody and network policy.

## Ownership examples

| Example | Protocol owner | Hatter treatment |
| --- | --- | --- |
| HTTP | IETF | exact external reference; no `http` entity or HTTP implementation vocabulary |
| Zixcel request/response schema | its Zixcel repository | exact external reference projected through a HAT ecosystem adapter |
| Crowsi transport | its Crowsi contract repository | opaque external reference; Crowsi retains addressing and policy |
| Hatter management JSONL | Hatter | Hatter-owned control protocol for CLI and Console parity |
| Codex-derived engine types | Hatter internal engine boundary | implementation vocabulary only; not a second public or semantic protocol |

Provider-specific terms may exist in the provider's own SemanticCatalog when
they describe domain meaning. They may not be copied into the foundation merely
because the provider uses a protocol.

## Validation rules

- reject unknown fields, zero revisions, endpoint credentials, query strings
  and fragments in protocol identity;
- compare protocol identity as the exact four-field reference, never by label,
  URI prefix, compatible-looking version or model inference;
- require every declared operation to exist in the same package;
- keep communication capabilities optional for operations that do not cross a
  separately constrained communication boundary;
- an accepted protocol requirement narrows provider selection but never grants
  installation, binding, placement, permission or execution authority;
- unresolved or multiple providers stay visible and fail closed.
