pub(crate) const OBSERVE: &[&str] = &[
    "compare-state",
    "inspect-aggregate",
    "inspect-metadata",
    "list-records",
    "read-summary",
    "review-evidence",
];

pub(crate) const PROPOSE: &[&str] = &[
    "compare-state",
    "inspect-aggregate",
    "inspect-metadata",
    "list-records",
    "propose-allocation",
    "propose-boundary",
    "propose-classification",
    "propose-obligation",
    "propose-publication",
    "propose-recommendation",
    "propose-scenario",
    "read-summary",
    "review-evidence",
];

pub(crate) const EXECUTE: &[&str] = &[
    "create-private-resource",
    "create-review-request",
    "create-work-item",
    "dispatch-operation",
    "delete-resource",
    "publish-artifact",
];

pub(crate) fn allowed(mode: &str) -> &'static [&'static str] {
    match mode {
        "observe" => OBSERVE,
        "propose" => PROPOSE,
        "execute" => EXECUTE,
        _ => &[],
    }
}
