# Catalog source shape: frame retirement

This is an unreleased source change. The package version remains 0.10.0; no
registry artifact or signed index is replaced by changing these source files.

HatSpec declares wire-bound operational actions, package identity, capabilities,
scope, grants and execution contracts. It no longer defines clause frames,
linguistic role specifications or a frame digest/validator. Language compilation
and adopted meaning belong to sem-lang; an operational declaration cannot grant
meaning adoption or execution authority by itself.

The physical catalog schema and closed Rust decoder both omit `frames`. An old
field, even `frames: []`, is an error. A decoder alias, optional legacy field or
second linguistic schema is deliberately not retained. Remaining term/display
metadata is not claimed to be a completed separation of every semantic concern.

## Producer and consumer obligations

1. Generate the current foundation and communication catalogs through the Rust
   producer APIs. Do not reuse an old serialized foundation digest.
2. Produce a fresh operational package catalog and recompute its digest, exact
   self-references and dependencies. The checked-in source-curator example is a
   development example, not a replacement for released bytes.
3. Validate the package and declared wire actions with the same source contract.
   Input/output, action ownership, capabilities, scope and grants still apply.
4. Exercise independent consumers and synthetic signed-selection journeys. The
   inner semantic catalog digest and the outer release-index hash are distinct.
5. Before registry distribution, choose a new immutable artifact identity through
   the release owner's process. Re-signing, release numbering, registry approval,
   deployment and activation are separate operations.

The old one-time JavaScript migration script generated linguistic frames and is
removed. It is not replaced by another compatibility converter or implicit
read-time rewrite. Source packages should be authored against the current model.

## Development evidence

`tests/catalog_frame_retirement.rs` verifies closed decoding, physical schema
removal and the canonical digest change caused even by removing an empty array.
It also checks that the development example references the current foundation
and passes package validation. Existing package, wire-operation and failure
envelope tests continue to exercise operational validation.

These owner tests alone are insufficient to accept an external producer cohort,
an installed consumer, a signed registry release or a production activation.
Keep those acceptances distinct and record the exact consumer source used.

The [semantic ownership inventory](semantic-ownership-acceptance.md) now records
the retained catalog/display/application boundaries and the exact two-producer
offline CLI/invocation acceptance cohort. It keeps real RPC, semantic migration,
publication and runtime authorization outstanding.
