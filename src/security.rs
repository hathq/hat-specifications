const MAX_DOCUMENT_BYTES: usize = 1_048_576;
const MAX_NESTING_DEPTH: usize = 32;

pub(crate) fn inspect(source: &str) -> Result<toml::Value, String> {
    if source.len() > MAX_DOCUMENT_BYTES {
        return Err("HAT document exceeds 1 MiB".to_owned());
    }
    let value: toml::Value = toml::from_str(source).map_err(|error| error.to_string())?;
    inspect_value(&value, 0)?;
    Ok(value)
}

pub(crate) fn inspect_json(source: &str) -> Result<serde_json::Value, String> {
    if source.len() > MAX_DOCUMENT_BYTES {
        return Err("HAT document exceeds 1 MiB".to_owned());
    }
    let value = serde_json::from_str(source).map_err(|error| error.to_string())?;
    inspect_json_value(&value, 0)?;
    Ok(value)
}

fn inspect_json_value(value: &serde_json::Value, depth: usize) -> Result<(), String> {
    if depth > MAX_NESTING_DEPTH {
        return Err("HAT document nesting exceeds 32 levels".to_owned());
    }
    match value {
        serde_json::Value::Object(object) => {
            for (key, value) in object {
                if credential_key(key) {
                    return Err("HAT documents must not contain credential fields".to_owned());
                }
                inspect_json_value(value, depth + 1)?;
            }
        }
        serde_json::Value::Array(items) => {
            for value in items {
                inspect_json_value(value, depth + 1)?;
            }
        }
        serde_json::Value::String(value) if raw_secret(value) => {
            return Err("HAT documents must not contain raw secret values".to_owned());
        }
        _ => {}
    }
    Ok(())
}

fn inspect_value(value: &toml::Value, depth: usize) -> Result<(), String> {
    if depth > MAX_NESTING_DEPTH {
        return Err("HAT document nesting exceeds 32 levels".to_owned());
    }
    match value {
        toml::Value::Table(table) => {
            for (key, value) in table {
                if credential_key(key) {
                    return Err("HAT documents must not contain credential fields".to_owned());
                }
                inspect_value(value, depth + 1)?;
            }
        }
        toml::Value::Array(items) => {
            for value in items {
                inspect_value(value, depth + 1)?;
            }
        }
        toml::Value::String(value) if raw_secret(value) => {
            return Err("HAT documents must not contain raw secret values".to_owned());
        }
        _ => {}
    }
    Ok(())
}

fn credential_key(key: &str) -> bool {
    let key = key
        .chars()
        .filter(char::is_ascii_alphanumeric)
        .flat_map(char::to_lowercase)
        .collect::<String>();
    [
        "apikey",
        "authorization",
        "bearertoken",
        "clientsecret",
        "connectionstring",
        "credential",
        "credentials",
        "password",
        "passwd",
        "privatekey",
        "refreshtoken",
        "secret",
        "token",
        "accesstoken",
    ]
    .contains(&key.as_str())
        || ["password", "secret", "token", "apikey", "privatekey"]
            .iter()
            .any(|suffix| key.ends_with(suffix))
}

fn raw_secret(value: &str) -> bool {
    let value = value.trim();
    (value.contains("-----BEGIN") && value.contains("PRIVATE KEY-----"))
        || value.starts_with("ghp_")
        || value.starts_with("github_pat_")
        || value.starts_with("sk-")
        || (value.starts_with("AKIA") && value.len() == 20)
        || value
            .strip_prefix("Bearer ")
            .is_some_and(|token| token.len() >= 16)
}

#[cfg(test)]
mod tests {
    use super::inspect;

    #[test]
    fn credentials_raw_keys_and_oversize_inputs_are_rejected() {
        assert!(inspect("token = \"no\"").is_err());
        assert!(inspect("name = \"ghp_0123456789abcdef\"").is_err());
        assert!(inspect(&" ".repeat(1_048_577)).is_err());
    }
}
