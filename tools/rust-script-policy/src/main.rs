//! Fail-closed guard for newly introduced first-party Python tooling.
//! Tests and implementation are intentionally std-only Rust.
use std::env;
use std::process::{Command, ExitCode};

fn is_python_tool_path(path: &str) -> bool {
    let lower = path.to_ascii_lowercase();
    if ![".py", ".pyi", ".ipynb"]
        .iter()
        .any(|extension| lower.ends_with(extension))
    {
        return false;
    }
    let parts: Vec<&str> = lower.split('/').collect();
    parts.iter().any(|part| {
        matches!(
            *part,
            "script" | "scripts" | "test" | "tests" | "tools" | "tooling" | "bin" | ".github"
        )
    })
}

fn is_command_file(path: &str) -> bool {
    let lower = path.to_ascii_lowercase();
    lower.starts_with(".github/workflows/")
        || lower
            .split('/')
            .any(|part| matches!(part, "script" | "scripts" | "bin"))
        || [".sh", ".bash", ".zsh", ".just", ".mk", ".make"]
            .iter()
            .any(|suffix| lower.ends_with(suffix))
        || lower.ends_with("/makefile")
        || lower == "makefile"
        || lower.ends_with("/justfile")
        || lower == "justfile"
}

fn is_identifier_byte(value: u8) -> bool {
    value.is_ascii_alphanumeric() || value == b'_'
}

fn has_word(text: &str, needle: &str) -> bool {
    let bytes = text.as_bytes();
    let word = needle.as_bytes();
    if bytes.len() < word.len() {
        return false;
    }
    for start in 0..=bytes.len() - word.len() {
        if &bytes[start..start + word.len()] != word {
            continue;
        }
        let end = start + word.len();
        if (start == 0 || !is_identifier_byte(bytes[start - 1]))
            && (end == bytes.len() || !is_identifier_byte(bytes[end]))
        {
            return true;
        }
    }
    false
}

fn invokes_python(added_line: &str) -> bool {
    let value = added_line.trim().to_ascii_lowercase();
    if value.is_empty() || value.starts_with('#') {
        return false;
    }
    if value.contains("setup-python@") {
        return true;
    }
    [
        "python", "python2", "python3", "pytest", "pip", "pip3", "poetry", "uv",
    ]
    .iter()
    .any(|needle| has_word(&value, needle))
        || value.contains(".py ")
        || value.ends_with(".py")
}

fn inspect_paths(paths: &[String]) -> Vec<String> {
    paths
        .iter()
        .filter(|path| is_python_tool_path(path))
        .map(|path| format!("{path}: Python test/tool file changed; use Rust"))
        .collect()
}

fn inspect_diff(diff: &str) -> Vec<String> {
    let mut errors = Vec::new();
    let mut path = "";
    for line in diff.lines() {
        if let Some(new_path) = line.strip_prefix("+++ b/") {
            path = new_path;
            continue;
        }
        if line == "+++ /dev/null" {
            path = "";
            continue;
        }
        if let Some(added) = line.strip_prefix('+') {
            if !path.is_empty() && is_command_file(path) && invokes_python(added) {
                errors.push(format!("{path}: added Python tooling invocation"));
            }
        }
    }
    errors
}

fn git(args: &[&str]) -> Result<Vec<u8>, String> {
    let output = Command::new("git")
        .args(args)
        .output()
        .map_err(|error| format!("unable to run git: {error}"))?;
    if !output.status.success() {
        return Err(format!(
            "git failed ({}): {}",
            output.status,
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    Ok(output.stdout)
}

fn valid_commit_sha(value: &str) -> bool {
    value.len() == 40 && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}

fn check(base: &str, head: &str) -> Result<(), String> {
    if !valid_commit_sha(base) || !valid_commit_sha(head) {
        return Err("base and head must be immutable 40-hex commit SHAs".to_owned());
    }
    let names = git(&[
        "diff",
        "--name-only",
        "-z",
        "--find-renames",
        "--diff-filter=ACMR",
        base,
        head,
        "--",
    ])?;
    let paths = names
        .split(|byte| *byte == 0)
        .filter(|entry| !entry.is_empty())
        .map(|entry| {
            std::str::from_utf8(entry)
                .map(str::to_owned)
                .map_err(|_| "non-UTF8 path in diff is not admissible".to_owned())
        })
        .collect::<Result<Vec<_>, _>>()?;
    let patch = git(&[
        "diff",
        "--no-ext-diff",
        "--no-color",
        "--unified=0",
        "--find-renames",
        base,
        head,
        "--",
    ])?;
    let patch = String::from_utf8(patch)
        .map_err(|_| "non-UTF8 diff is not admissible".to_owned())?;
    let mut errors = inspect_paths(&paths);
    errors.extend(inspect_diff(&patch));
    if !errors.is_empty() {
        errors.sort();
        errors.dedup();
        return Err(errors.join("\n"));
    }
    Ok(())
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();
    let [base_flag, base, head_flag, head] = args.as_slice() else {
        eprintln!("usage: rust-script-policy --base <commit-sha> --head <commit-sha>");
        return ExitCode::FAILURE;
    };
    if base_flag != "--base" || head_flag != "--head" {
        eprintln!("expected --base SHA --head SHA");
        return ExitCode::FAILURE;
    }
    match check(base, head) {
        Ok(()) => {
            println!("PASS: changed first-party scripts and tests are Rust-first");
            ExitCode::SUCCESS
        }
        Err(message) => {
            eprintln!("FAIL: Rust-first tooling policy:\n{message}");
            ExitCode::FAILURE
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_python_test_and_tool_files() {
        for name in [
            "scripts/validate.py",
            "tests/generator_test.py",
            "tools/audit.pyi",
            ".github/check.ipynb",
            "bin/run.py",
        ] {
            assert!(is_python_tool_path(name), "{name}");
        }
    }

    #[test]
    fn does_not_reject_generated_python_client_adapters() {
        assert!(!is_python_tool_path(
            "generated/python/akrion_sim_interfaces.py"
        ));
        assert!(!is_python_tool_path("src/application/handler.py"));
        assert!(!is_python_tool_path("tests/contract.rs"));
    }

    #[test]
    fn detects_added_workflow_and_shell_invocations() {
        let diff = "diff --git a/.github/workflows/test.yml b/.github/workflows/test.yml\n\
                    +++ b/.github/workflows/test.yml\n\
                    @@ -0,0 +1,2 @@\n\
                    +      run: python3 scripts/old.py\n\
                    +      uses: actions/setup-python@abcedf\n\
                    diff --git a/scripts/check.sh b/scripts/check.sh\n\
                    +++ b/scripts/check.sh\n\
                    @@ -0,0 +1 @@\n\
                    +pytest -q\n";
        assert_eq!(inspect_diff(diff).len(), 3);
    }

    #[test]
    fn allows_rust_commands_and_unrelated_markdown() {
        let diff = "+++ b/.github/workflows/test.yml\n\
                    +      run: cargo test --locked\n\
                    +# Migrate historical Python code separately\n\
                    +++ b/README.md\n\
                    +We prefer Rust tests instead of Python scripts.\n";
        assert!(inspect_diff(diff).is_empty());
    }

    #[test]
    fn ignores_deleted_lines() {
        let diff = "+++ b/.github/workflows/test.yml\n\
                    -      run: python3 tests/legacy.py\n\
                    +      run: cargo test --locked\n";
        assert!(inspect_diff(diff).is_empty());
    }

    #[test]
    fn rejects_case_insensitive_python_commands() {
        assert!(invokes_python(" run: PYTEST -q"));
        assert!(invokes_python(" python -m unittest"));
        assert!(invokes_python(" uses: actions/setup-python@deadbeef"));
        assert!(invokes_python(" run: pip install jsonschema"));
        assert!(!invokes_python(" run: cargo test --all"));
    }

    #[test]
    fn only_accepts_exact_commit_ids() {
        assert!(valid_commit_sha(&"a".repeat(40)));
        assert!(!valid_commit_sha("main"));
        assert!(!valid_commit_sha(&"z".repeat(40)));
        assert!(!valid_commit_sha(&"0".repeat(39)));
    }

    #[test]
    fn flags_a_changed_python_script_instead_of_existing_debt() {
        let paths = vec!["scripts/legacy.py".to_string(), "src/lib.rs".to_string()];
        let findings = inspect_paths(&paths);
        assert_eq!(findings.len(), 1);
        assert!(findings[0].contains("scripts/legacy.py"));
    }
}
