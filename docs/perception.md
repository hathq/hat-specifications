# Perception wiring (0.10.0)

Status: typed contract and pure bounded planning tested; not yet registered as
an operational record or installed into the Hatter product owner.

PerceptionBinding connects an existing person/role, an adopted sem-lang sense
definition and an external provider/account/scope to explicit interpretation
routes. It does not define biological senses, mail meaning or accounting rules.
Those meanings remain sem-lang definitions and HAT-owned application bindings.

```text
person / role
  -> adopted sense reference
  -> provider + account + source scope + input contract
  -> exact observation source
  -> zero execution until ordinary Work admission
  -> each selected HAT + RuntimeRole + sem-lang binding
  -> original semantic observation / explicit admission / proposed Tasks
```

`plan_perception` accepts enabled bindings with 1..32 distinct interpretation
routes. It checks all exact provider/account/scope/schema references, revision
and digest syntax, canonicalizes route ordering, and returns a deterministic
request key. Provider facts must be verified by their owning service before
this function is called; structurally valid references are not proof of truth.
The proposal is NOT an ExecutionGrant or a completed semantic inference.

An input re-delivery must retain the original immutable observation and capture
time. Reusing the same input/binding produces the same key. A different account,
source revision, interpretation binding or observed location produces a distinct
key; the original Work owner remains responsible for durable replay receipts.

The source owner retains the original input. The binding and plan contain only
exact references, not copies of mail bodies, credentials or semantic memory.
The next Work must recheck revocation before accessing protected input or causing
an effect. Read permission must not become mail-send or ledger-post permission.

Time of local reception, source occurrence time and source-backed physical
location are separate. Unknown occurrence/location stay absent. Scene x/y/z,
mail sender domains and installation locale never supply missing physical facts.

## Required product integration (not covered by these tests)

1. Register exact bindings under the Hatter Graph/control owner using existing
   revision/CAS/publication/retention, not a new settings database.
2. Discover actual installed input HATs and bind their signed input schemas.
3. Admit the account-specific source observation through its owning service.
4. Submit each explicit route through existing bounded Work/scheduler, restore
   each role's sem-lang context and use the admitted local inference runtime.
5. Keep conflicting outputs as candidates. Only an explicit owner decision may
   adopt an interpretation; only separately authorized Tasks may cause effects.
6. Publish source-backed connection/observation/proposal state to the spatial UI.
   A planned binding must not be rendered as a working sense or completed task.

`tests/perception.rs` covers fan-out, exact replay keys, multiple-account and
scope refusal, disabled connections, duplicate/excess routes, reference
validation, unknown credential/payload fields and independent time/place refs.
