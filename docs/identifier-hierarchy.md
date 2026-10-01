# Identifier hierarchy and contract boundaries

This document defines how Hatter represents every ecosystem identifier without
confusing a URI's spelling with its semantic meaning.

## Normative distinction

An identifier is an immutable, exact IRI. Its URI path is not tokenized to
infer meaning. A contract descriptor supplies the owner, kind, revision,
specification digest and an optional semantic parent. The semantic parent and
its `is_a`, `domain` and `range` relations are owned by a SemanticCatalog.

This follows the generic URI model: slash separates path segments, but a path
segment is opaque unless the scheme defines additional semantics ([RFC 3986,
section 3.3](https://www.rfc-editor.org/rfc/rfc3986.html#section-3.3)). RDF
vocabularies are sets of IRIs and their hierarchy is expressed by graph
relations, not by splitting identifier strings ([RDF 1.2 Concepts](https://www.w3.org/TR/rdf12-concepts/)).
JSON Schema `$id` is a schema identifier/base URI, so changing it requires
updating every `$ref` and the published digest ([JSON Schema 2020-12](https://json-schema.org/understanding-json-schema/structuring)).

## Universal descriptor

Every scanned reference is projected to this logical shape:

```json
{
  "id": "hathq://hat/schema/action/result/v2",
  "owner_repository_id": "hat-specifications",
  "kind": "schema",
  "revision": 1,
  "specification_ref": "hathq://hat/specification/action/result/v1",
  "semantic_parent": "hathq://vocabulary/action/result/v1",
  "hierarchy": ["schema", "action", "result"],
  "identity_mode": "exact",
  "status": "active"
}
```

`hierarchy` is a declared projection for graph navigation and UI filtering;
`id` remains the wire identity. When no owner declaration proves a split, the
last segment is represented as one `opaque` node rather than guessed words.

## Namespace tree

The common tree is:

```text
scheme
└── authority / owner
    └── kind (schema, protocol, vocabulary, ledger, transport, resource)
        └── declared semantic family
            └── exact contract or resource identity
                └── revision and immutable digest
```

`hathq://hat/...` is the shared HatSpec contract namespace. Hatter and its
Console use separate local authorities under the same scheme, while package
owners use their own authorities. Crowsi, Zixcel, Estate, iHAT and other schemes remain owned by
their repositories and are referenced through an adapter descriptor.

The existing `hathq://vocabulary/...` authority is treated as a legacy
vocabulary namespace. It is a migration candidate, not an alias: moving it to
`hathq://hat/vocabulary/...` requires a new contract revision and an atomic
reference, signature and digest update.

## Migration rules

1. A regular expression may find candidate compound segments, but it may not
   rewrite source text directly.
2. Only an owner-declared mapping with one unambiguous SemanticCatalog parent
   may receive a hierarchical display path.
3. A URI path change is a new contract identity. Publish a new revision and
   update schemas, references, signatures and digests atomically; do not add an
   implicit compatibility alias.
4. External, resource, evidence, repository, workspace, product and proper
   names stay opaque. Their spelling is controlled by the owning service.
5. Malformed values, templates and test fixtures are repaired or explicitly
   marked as fixtures; they are never inferred into production vocabulary.

## Interface ownership

- `SemanticCatalog`: term definitions and graph hierarchy.
- `ContractDescriptor`: owner, kind, exact identity and declared hierarchy.
- JSON Schema: payload shape and `$id` identity.
- Communication capability: exact external protocol reference; no address or
  credentials.
- Local registry ledger: provenance, source, digest and verification state.
- Hatter: resolution and projection only; it does not copy provider vocabularies.

The debugger may expose these relations in its graph projection, but the graph
is a development view and is never the canonical contract store.
