// Added by the HAT Specifications project, 2026.
// Purpose: perform the one-way package-v1 vocabulary to package-v2 catalog migration.

import { createHash } from 'node:crypto'
import { readFile, writeFile } from 'node:fs/promises'

const foundation = JSON.parse(await readFile(process.env.HAT_FOUNDATION_CATALOG, 'utf8'))
const files = process.argv.slice(2)
if (!files.length) throw new Error('package paths are required')

const canonical = value => {
  if (Array.isArray(value)) return `[${value.map(canonical).join(',')}]`
  if (value !== null && typeof value === 'object') return `{${Object.keys(value).sort()
    .map(key => `${JSON.stringify(key)}:${canonical(value[key])}`).join(',')}}`
  return JSON.stringify(value)
}
const hash = value => createHash('sha256').update(canonical(value)).digest('hex')
const kind = value => ({ entity: 'entity-type', event: 'event-type', state: 'state-type' }[value] ?? value)

for (const file of files) {
  const source = JSON.parse(await readFile(file, 'utf8'))
  if (!['hathq://hat/package/v1', 'hathq://hat/package/v2'].includes(source.schema)) {
    throw new Error(`${file}: expected package v1 or v2`)
  }
  const operationInputs = new Map(source.operations.map(operation => [operation.id, operation.input_schema]))
  const legacyTerms = source.vocabulary?.terms
  const terms = legacyTerms ? legacyTerms.map(term => {
    const reference = {
      catalog_id: source.repository_id,
      catalog_digest_sha256: '',
      term_id: term.id,
      version: 1,
      definition_digest_sha256: '',
      kind: kind(term.kind),
    }
    const definition = {
      reference,
      wire_schema: operationInputs.get(term.id) ?? term.wire_schema,
      semantic_icon: term.semantic_icon ?? 'generic',
      dependencies: [], is_a: [], domain: [], range: [],
    }
    reference.definition_digest_sha256 = hash(definition)
    return definition
  }) : structuredClone(source.catalog.terms)
  for (const term of terms) {
    term.reference.catalog_digest_sha256 = ''
    term.reference.definition_digest_sha256 = ''
    for (const relation of ['dependencies', 'is_a', 'domain', 'range']) {
      for (const reference of term[relation]) reference.catalog_digest_sha256 = ''
    }
    term.reference.definition_digest_sha256 = hash(term)
  }
  const references = new Map(terms.map(term => [term.reference.term_id, term.reference]))
  const frames = source.operations.map(operation => {
    const frame = {
      frame_id: `${operation.procedure.id}-frame`,
      definition_digest_sha256: '',
      predicate: structuredClone(references.get(operation.id)),
      roles: [
        { role: 'actor', required: true, accepted_kinds: ['entity-type'] },
        { role: 'object', required: true, accepted_kinds: ['concept'] },
      ],
    }
    frame.definition_digest_sha256 = hash(frame)
    return frame
  })
  const catalog = {
    schema: 'hathq://hat/semantic-catalog/v1',
    identity: { catalog_id: source.repository_id, version: 1, digest_sha256: '' },
    owner_repository_id: source.repository_id,
    foundation: false,
    dependencies: [foundation.identity],
    incompatibilities: source.catalog?.incompatibilities ?? [],
    terms, lexicalizations: [], frames,
  }
  const catalogDigest = hash(catalog)
  catalog.identity.digest_sha256 = catalogDigest
  for (const term of catalog.terms) term.reference.catalog_digest_sha256 = catalogDigest
  for (const frame of catalog.frames) frame.predicate.catalog_digest_sha256 = catalogDigest
  const { vocabulary: _removed, catalog: _oldCatalog, ...rest } = source
  const target = { ...rest, schema: 'hathq://hat/package/v2', catalog }
  await writeFile(file, `${JSON.stringify(target, null, 2)}\n`)
}
