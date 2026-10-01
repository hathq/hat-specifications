# HatSpec responsibility boundary

- Package version remains 0.10.0 until the user explicitly changes it.
- sem-lang owns language meaning, canonical IR, compilation outcomes and memory rules.
- HatSpec owns HAT capabilities, package identity, scope, grant and execution contracts.
- Never recreate a semantic inference packet, text resolver or language compiler here.
- A Complete language result grants no operational authority.
- Do not restore removed wire schemas, aliases or compatibility decoders.
- Catalog/clause/exchange linguistic responsibilities still awaiting migration must
  be reported as outstanding, not described as the new language foundation.
- Distinguish source tests, immutable registry artifacts and product E2E acceptance.
