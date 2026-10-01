// Added by the HAT Specifications project, 2026.
// Purpose: publish the one immutable semantic foundation shared by independently owned Hatters.

use crate::{
    SEMANTIC_CATALOG_SCHEMA, SemanticCatalog, SemanticCatalogReference, SemanticLexicalization,
    SemanticTermDefinition, SemanticTermKind, SemanticTermReference, VocabularyIcon,
    semantic_catalog_digest, semantic_term_definition_digest, validate_semantic_catalog,
};
use std::collections::{BTreeMap, BTreeSet};

const CATALOG_ID: &str = "hathq.foundation";

/// Returns the exact, digest-bound foundation catalog required before an optional HAT is installed.
///
/// # Panics
///
/// Panics only when the statically reviewed foundation definitions violate
/// their dependency order or digest/validation invariants.
#[must_use]
pub fn foundation_semantic_catalog() -> SemanticCatalog {
    let mut references = BTreeMap::<&str, SemanticTermReference>::new();
    let mut terms = Vec::new();
    let mut lexicalizations = Vec::new();
    for spec in specifications() {
        let dependencies = spec
            .dependencies
            .iter()
            .map(|id| {
                references
                    .get(*id)
                    .expect("foundation dependency order")
                    .clone()
            })
            .collect();
        let mut definition = SemanticTermDefinition {
            reference: SemanticTermReference {
                catalog_id: CATALOG_ID.into(),
                catalog_digest_sha256: String::new(),
                term_id: spec.id.into(),
                version: 1,
                definition_digest_sha256: String::new(),
                kind: spec.kind,
            },
            wire_schema: None,
            semantic_icon: spec.icon,
            dependencies,
            is_a: BTreeSet::new(),
            domain: BTreeSet::new(),
            range: BTreeSet::new(),
        };
        definition.reference.definition_digest_sha256 =
            semantic_term_definition_digest(&definition).expect("foundation term digest");
        references.insert(spec.id, definition.reference.clone());
        for (locale, preferred) in [("en", spec.en), ("ja", spec.ja)] {
            lexicalizations.push(SemanticLexicalization {
                term: definition.reference.clone(),
                locale: locale.into(),
                preferred: preferred.into(),
                search_aliases: BTreeSet::new(),
            });
        }
        terms.push(definition);
    }
    let mut catalog = SemanticCatalog {
        schema: SEMANTIC_CATALOG_SCHEMA.into(),
        identity: SemanticCatalogReference {
            catalog_id: CATALOG_ID.into(),
            version: 1,
            digest_sha256: String::new(),
        },
        owner_repository_id: "hat-specifications".into(),
        foundation: true,
        dependencies: Vec::new(),
        incompatibilities: Vec::new(),
        terms,
        lexicalizations,
        frames: Vec::new(),
    };
    let digest = semantic_catalog_digest(&catalog).expect("foundation catalog digest");
    catalog.identity.digest_sha256.clone_from(&digest);
    for definition in &mut catalog.terms {
        set_catalog_digest(&mut definition.reference, &digest);
        set_catalog_digest_set(&mut definition.dependencies, &digest);
        set_catalog_digest_set(&mut definition.is_a, &digest);
        set_catalog_digest_set(&mut definition.domain, &digest);
        set_catalog_digest_set(&mut definition.range, &digest);
    }
    for lexicalization in &mut catalog.lexicalizations {
        set_catalog_digest(&mut lexicalization.term, &digest);
    }
    validate_semantic_catalog(&catalog).expect("valid foundation catalog");
    catalog
}

fn set_catalog_digest(reference: &mut SemanticTermReference, digest: &str) {
    reference.catalog_digest_sha256 = digest.into();
}

fn set_catalog_digest_set(values: &mut BTreeSet<SemanticTermReference>, digest: &str) {
    *values = std::mem::take(values)
        .into_iter()
        .map(|mut value| {
            set_catalog_digest(&mut value, digest);
            value
        })
        .collect();
}

struct Specification {
    id: &'static str,
    kind: SemanticTermKind,
    icon: VocabularyIcon,
    dependencies: &'static [&'static str],
    en: &'static str,
    ja: &'static str,
}

fn specifications() -> Vec<Specification> {
    let mut result = core_specifications();
    result.extend(world_specifications());
    result
}

#[allow(clippy::too_many_lines)] // Reviewed order is the dependency topology of one immutable catalog.
fn core_specifications() -> Vec<Specification> {
    use SemanticTermKind::{Action, Concept, EventType, StateType, Unit};
    use VocabularyIcon::{
        Action as ActionIcon, Decision, Entity, Event, Evidence, Generic, Identity, Language,
        Place, Relation, Service, Source, State, Time, Work,
    };
    vec![
        spec("core.entity", Concept, Entity, &[], "Entity", "実体"),
        spec(
            "core.identifier",
            Concept,
            Identity,
            &[],
            "Identifier",
            "識別子",
        ),
        spec("core.type", Concept, Generic, &[], "Type", "型"),
        spec("core.relation", Concept, Relation, &[], "Relation", "関係"),
        spec("core.assertion", Concept, Generic, &[], "Assertion", "表明"),
        spec("core.event", Concept, Event, &[], "Event", "出来事"),
        spec("core.state", Concept, State, &[], "State", "状態"),
        spec("core.evidence", Concept, Evidence, &[], "Evidence", "根拠"),
        spec("core.source", Concept, Source, &[], "Source", "情報源"),
        spec("core.quantity", Concept, Generic, &[], "Quantity", "量"),
        spec("core.unit", Unit, Generic, &[], "Unit", "単位"),
        spec(
            "core.condition",
            Concept,
            State,
            &["core.state"],
            "Condition",
            "条件",
        ),
        spec(
            "core.role",
            Concept,
            Relation,
            &["core.relation"],
            "Role",
            "役割",
        ),
        spec(
            "core.preference",
            Concept,
            Generic,
            &[],
            "Preference",
            "選好",
        ),
        spec(
            "core.action",
            Action,
            ActionIcon,
            &["core.event"],
            "Action",
            "操作",
        ),
        spec("core.goal", Concept, Work, &[], "Goal", "目標"),
        spec(
            "core.desired-state",
            Concept,
            State,
            &["core.state"],
            "Desired state",
            "望ましい状態",
        ),
        spec(
            "core.commitment",
            Concept,
            Decision,
            &["core.goal", "core.desired-state"],
            "Commitment",
            "コミットメント",
        ),
        spec(
            "core.constraint",
            Concept,
            State,
            &["core.condition"],
            "Constraint",
            "制約",
        ),
        spec("core.plan", Concept, Work, &["core.goal"], "Plan", "計画"),
        spec("core.task", Concept, Work, &["core.plan"], "Task", "タスク"),
        spec(
            "core.capability",
            Concept,
            ActionIcon,
            &["core.action"],
            "Capability",
            "能力",
        ),
        spec(
            "core.permission",
            Concept,
            Decision,
            &["core.capability"],
            "Permission",
            "許可",
        ),
        spec(
            "core.delegation",
            Concept,
            Decision,
            &["core.permission"],
            "Delegation",
            "委譲",
        ),
        spec(
            "core.result",
            Concept,
            Evidence,
            &["core.event"],
            "Result",
            "結果",
        ),
        spec(
            "core.service",
            Concept,
            Service,
            &["core.entity"],
            "Service",
            "サービス",
        ),
        spec(
            "core.provider",
            Concept,
            Service,
            &["core.service", "core.capability"],
            "Provider",
            "提供手段",
        ),
        spec(
            "core.execution",
            EventType,
            Event,
            &["core.action", "core.provider"],
            "Execution",
            "実行",
        ),
        spec(
            "core.verification",
            EventType,
            Evidence,
            &["core.result", "core.desired-state"],
            "Verification",
            "検証",
        ),
        spec("core.language", Concept, Language, &[], "Language", "言語"),
        spec("core.time.instant", Concept, Time, &[], "Instant", "時点"),
        spec("core.time.interval", Concept, Time, &[], "Interval", "期間"),
        spec(
            "core.time.duration",
            Concept,
            Time,
            &[],
            "Duration",
            "継続時間",
        ),
        spec(
            "core.time.recurrence",
            Concept,
            Time,
            &[],
            "Recurrence",
            "繰り返し",
        ),
        spec(
            "core.space.position",
            Concept,
            Place,
            &[],
            "Position",
            "位置",
        ),
        spec("core.space.place", Concept, Place, &[], "Place", "場所"),
        spec("core.space.region", Concept, Place, &[], "Region", "地域"),
        spec(
            "core.space.coordinate",
            Concept,
            Place,
            &[],
            "Coordinate",
            "座標",
        ),
        spec(
            "core.event.observation",
            EventType,
            Event,
            &["core.event"],
            "Observation",
            "観測",
        ),
        spec(
            "core.event.action.requested",
            EventType,
            Event,
            &["core.execution"],
            "Action requested",
            "操作を要求した",
        ),
        spec(
            "core.event.action.started",
            EventType,
            Event,
            &["core.execution"],
            "Action started",
            "操作を開始した",
        ),
        spec(
            "core.event.action.waiting",
            EventType,
            Event,
            &["core.execution"],
            "Action waiting",
            "操作が待機している",
        ),
        spec(
            "core.event.action.unresolved",
            EventType,
            Event,
            &["core.execution"],
            "Action unresolved",
            "操作を解決できていない",
        ),
        spec(
            "core.event.action.completed",
            EventType,
            Event,
            &["core.execution", "core.result"],
            "Action completed",
            "操作を完了した",
        ),
        spec(
            "core.event.action.failed",
            EventType,
            Event,
            &["core.execution", "core.result"],
            "Action failed",
            "操作に失敗した",
        ),
        spec(
            "core.event.action.cancelled",
            EventType,
            Event,
            &["core.execution"],
            "Action cancelled",
            "操作を取り消した",
        ),
        spec(
            "core.action.invoke",
            Action,
            VocabularyIcon::Action,
            &["core.action"],
            "Invoke exact action",
            "正確な操作を呼び出す",
        ),
        spec(
            "core.state.observed",
            StateType,
            State,
            &["core.state"],
            "Observed",
            "観測済み",
        ),
    ]
}

#[allow(clippy::too_many_lines)] // Reviewed order is the dependency topology of one immutable catalog.
fn world_specifications() -> Vec<Specification> {
    use SemanticTermKind::{Concept, EntityType};
    use VocabularyIcon::{
        Asset, Biology, Health, Identity, Jurisdiction, Language, Organization, Person, Place,
        Relation, Resource, Service,
    };
    vec![
        spec(
            "world.person",
            EntityType,
            Person,
            &["core.entity"],
            "Person",
            "人",
        ),
        spec(
            "world.person.identity",
            EntityType,
            Identity,
            &["world.person", "core.identifier"],
            "Personal identity",
            "本人性",
        ),
        spec(
            "world.person.biology",
            EntityType,
            Health,
            &["world.person"],
            "Human biology",
            "人の生物学的構造",
        ),
        spec(
            "world.person.relationship",
            Concept,
            Relation,
            &["world.person", "core.relation"],
            "Human relationship",
            "人の関係と立場",
        ),
        spec(
            "world.person.role",
            Concept,
            Relation,
            &["world.person", "core.role"],
            "Person role",
            "人の役割",
        ),
        spec(
            "world.person.preference",
            Concept,
            Identity,
            &["world.person", "core.preference"],
            "Person preference",
            "人の選好",
        ),
        spec(
            "world.person.goal",
            Concept,
            Identity,
            &["world.person", "core.goal"],
            "Person goal",
            "人の目標",
        ),
        spec(
            "world.body",
            EntityType,
            Biology,
            &["world.person.biology", "core.entity"],
            "Human body",
            "人体",
        ),
        spec(
            "world.body.part",
            EntityType,
            Biology,
            &["world.body"],
            "Body part",
            "身体部位",
        ),
        spec(
            "world.organization",
            EntityType,
            Organization,
            &["core.entity"],
            "Organization",
            "組織",
        ),
        spec(
            "world.organization.role",
            Concept,
            Relation,
            &["world.organization", "core.role"],
            "Organization role",
            "組織上の役割",
        ),
        spec(
            "world.institution",
            EntityType,
            Jurisdiction,
            &["world.organization"],
            "Institution",
            "制度機関",
        ),
        spec(
            "world.jurisdiction",
            EntityType,
            Jurisdiction,
            &["world.institution", "core.space.region"],
            "Jurisdiction",
            "管轄",
        ),
        spec(
            "world.language",
            EntityType,
            Language,
            &["core.language"],
            "Human language",
            "人間の言語",
        ),
        spec(
            "world.place",
            EntityType,
            Place,
            &["core.space.place"],
            "Real-world place",
            "現実世界の場所",
        ),
        spec(
            "world.resource",
            EntityType,
            Resource,
            &["core.entity"],
            "Resource",
            "資源",
        ),
        spec(
            "world.physical-object",
            EntityType,
            Resource,
            &["world.resource", "core.space.position"],
            "Physical object",
            "物理的な物",
        ),
        spec(
            "world.digital-resource",
            EntityType,
            Resource,
            &["world.resource"],
            "Digital resource",
            "デジタル資源",
        ),
        spec(
            "world.service",
            EntityType,
            Service,
            &["core.service", "world.organization"],
            "Provided service",
            "提供サービス",
        ),
        spec(
            "world.asset",
            EntityType,
            Asset,
            &["world.resource"],
            "Asset",
            "資産",
        ),
    ]
}

fn spec(
    id: &'static str,
    kind: SemanticTermKind,
    icon: VocabularyIcon,
    dependencies: &'static [&'static str],
    en: &'static str,
    ja: &'static str,
) -> Specification {
    Specification {
        id,
        kind,
        icon,
        dependencies,
        en,
        ja,
    }
}
