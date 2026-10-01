use std::collections::{BTreeMap, BTreeSet};

use crate::HatCompositionMember;

pub(crate) fn validate_edges(
    members: &BTreeMap<&String, &HatCompositionMember>,
    findings: &mut Vec<String>,
) {
    for (repository_id, member) in members {
        for dependency in &member.dependency_repository_ids {
            if dependency == *repository_id || !members.contains_key(dependency) {
                findings.push("composition dependency is missing or self-referential".into());
            }
        }
        for incompatible in &member.incompatible_repository_ids {
            if members.contains_key(incompatible) {
                findings.push("composition selects incompatible repositories".into());
            }
        }
    }
    for repository_id in members.keys() {
        if cyclic(
            repository_id,
            members,
            &mut BTreeSet::new(),
            &mut BTreeSet::new(),
        ) {
            findings.push("composition dependency graph is cyclic".into());
            break;
        }
    }
}

fn cyclic<'a>(
    id: &'a String,
    members: &BTreeMap<&'a String, &'a HatCompositionMember>,
    visited: &mut BTreeSet<&'a String>,
    active: &mut BTreeSet<&'a String>,
) -> bool {
    if active.contains(id) {
        return true;
    }
    if !visited.insert(id) {
        return false;
    }
    active.insert(id);
    let found = members.get(id).is_some_and(|member| {
        member
            .dependency_repository_ids
            .iter()
            .any(|dependency| cyclic(dependency, members, visited, active))
    });
    active.remove(id);
    found
}
