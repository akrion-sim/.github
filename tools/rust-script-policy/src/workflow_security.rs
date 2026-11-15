//! Rust replacement for the legacy Python workflow pin validator.
//! Checks repository-local GitHub workflow files without YAML execution.
use std::fs;
use std::path::Path;

fn use_reference(line: &str) -> Option<&str> {
    let mut rest = line.trim_start();
    if rest.starts_with('#') {
        return None;
    }
    if let Some(after_dash) = rest.strip_prefix('-') {
        rest = after_dash.trim_start();
    }
    let rest = rest.strip_prefix("uses:")?.trim_start();
    rest.split(|value: char| value.is_whitespace() || value == '#')
        .next()
        .map(|value| value.trim_matches(['"', '\'']))
        .filter(|value| !value.is_empty())
}

fn valid_sha(value: &str) -> bool {
    value.len() == 40 && value.bytes().all(|part| part.is_ascii_hexdigit())
}

pub fn validate_text(path: &str, source: &str) -> Vec<String> {
    let mut errors = Vec::new();
    for (index, line) in source.lines().enumerate() {
        let Some(reference) = use_reference(line) else {
            continue;
        };
        // Exact parity with the legacy validator: local and docker actions
        // are not treated as GitHub SHA references by this specific checker.
        if reference.starts_with("./") || reference.starts_with("docker://") {
            continue;
        }
        let Some((action, revision)) = reference.rsplit_once('@') else {
            errors.push(format!(
                "{path}:{}: remote action has no immutable ref",
                index + 1
            ));
            continue;
        };
        if action.is_empty() || !valid_sha(revision) {
            errors.push(format!(
                "{path}:{}: remote action is not pinned to a full commit SHA",
                index + 1
            ));
        }
    }
    errors
}

pub fn verify_repository() -> Result<(), String> {
    let root = Path::new(".github/workflows");
    let files = fs::read_dir(root)
        .map_err(|error| format!("cannot read {}: {error}", root.display()))?;
    let mut paths = Vec::new();
    for file in files {
        let path = file.map_err(|error| format!("cannot read directory entry: {error}"))?.path();
        if path.is_file()
            && matches!(path.extension().and_then(|ext| ext.to_str()), Some("yml" | "yaml"))
        {
            paths.push(path);
        }
    }
    paths.sort();
    let mut failures = Vec::new();
    for path in paths {
        let source = fs::read_to_string(&path)
            .map_err(|error| format!("cannot read {}: {error}", path.display()))?;
        failures.extend(validate_text(&path.display().to_string(), &source));
    }
    if failures.is_empty() {
        Ok(())
    } else {
        Err(failures.join("\n"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_full_sha_and_local_actions() {
        let content = format!(
            "jobs:\n  steps:\n    - uses: actions/checkout@{}\n    - uses: ./local\n",
            "f".repeat(40)
        );
        assert!(validate_text("test.yml", &content).is_empty());
    }

    #[test]
    fn rejects_mutable_branches_and_missing_references() {
        let content = "  - uses: actions/checkout@v4\n  - uses: owner/action\n";
        assert_eq!(validate_text("test.yml", content).len(), 2);
    }

    #[test]
    fn ignores_comments_and_parses_quoted_uses() {
        let content = format!(
            "# uses: owner/action@main\n - uses: 'owner/action@{}' # stable\n",
            "a".repeat(40)
        );
        assert!(validate_text("test.yml", &content).is_empty());
    }

    #[test]
    fn fails_closed_on_malformed_sha_and_empty_action() {
        let content = format!(
            "uses: owner/action@{}\nuses: @{}\n",
            "g".repeat(40),
            "b".repeat(40)
        );
        assert_eq!(validate_text("test.yml", &content).len(), 2);
    }

    #[test]
    fn keeps_legacy_local_and_docker_handling_explicit() {
        assert!(validate_text("test.yml", "uses: ./action\nuses: docker://image:tag\n").is_empty());
    }

    #[test]
    fn parse_only_true_uses_keys() {
        assert!(use_reference("    - name: uses: foo").is_none());
        assert!(use_reference(" # - uses: owner/action@v1").is_none());
        assert!(use_reference(" - uses: owner/action@v1").is_some());
    }
}
