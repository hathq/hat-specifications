mod cli_input;
mod commands;

use hat_specifications::{RESULT_SCHEMA, parse_manifest};
use serde::Serialize;
use std::{env, process::ExitCode};

#[derive(Serialize)]
struct ResultEnvelope {
    schema: &'static str,
    command: String,
    ok: bool,
    external_actions: bool,
    result: serde_json::Value,
    findings: Vec<String>,
}

fn main() -> ExitCode {
    let arguments = env::args().skip(1).collect::<Vec<_>>();
    let result = commands::run(&arguments);
    let ok = result.ok;
    println!(
        "{}",
        serde_json::to_string_pretty(&result).expect("result is serializable")
    );
    if ok {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(2)
    }
}

fn load_manifest(path: &str) -> Result<hat_specifications::HatManifest, String> {
    cli_input::read_bounded_utf8(path).and_then(|source| parse_manifest(&source))
}

fn envelope(
    command: &str,
    ok: bool,
    result: serde_json::Value,
    findings: Vec<String>,
) -> ResultEnvelope {
    ResultEnvelope {
        schema: RESULT_SCHEMA,
        command: command.to_string(),
        ok,
        external_actions: false,
        result,
        findings,
    }
}
