//! Build script for mcp-srv-korad-KD3005P.
//!
//! Computes the effective version string and exposes it as the
//! `BUILD_VERSION` environment variable for use with `env!()` at compile time.
//!
//! Logic:
//! - If the current commit is tagged: validate the tag matches `CARGO_PKG_VERSION`;
//!   raise a hard build error if they differ.
//! - If the current commit is NOT tagged: append `-dev-<short-hash>` to the
//!   Cargo.toml version.
//! - If git is unavailable: fall back silently to the plain Cargo.toml version.

use std::process::Command;

fn main() {
    // Re-run this script whenever HEAD moves or a new tag is created.
    println!("cargo:rerun-if-changed=.git/HEAD");
    println!("cargo:rerun-if-changed=.git/refs/tags");

    let cargo_version = env!("CARGO_PKG_VERSION");
    let version = compute_version(cargo_version);

    println!("cargo:rustc-env=BUILD_VERSION={version}");
}

/// Compute the final version string by consulting git.
///
/// Returns the plain `cargo_version` when git is unavailable.
fn compute_version(cargo_version: &str) -> String {
    // Try to get the tag that exactly points to HEAD.
    let exact_tag = run_git(&["describe", "--tags", "--exact-match", "HEAD"]);

    match exact_tag {
        Some(tag) => {
            // Normalize: some projects prefix tags with 'v' (e.g. "v1.0.0").
            let tag_version = tag.trim_start_matches('v');

            if tag_version != cargo_version {
                // Hard error: the git tag and Cargo.toml version are out of sync.
                panic!(
                    "\n\n\
                    =====================================================\n\
                    VERSION MISMATCH\n\
                    Git tag    : {tag}\n\
                    Cargo.toml : {cargo_version}\n\
                    Fix: update Cargo.toml version to match the git tag,\n\
                         or move the tag to the correct commit.\n\
                    =====================================================\n"
                );
            }

            // Tag matches Cargo.toml — use the clean version.
            cargo_version.to_string()
        }
        None => {
            // Not on a tag. Attempt to append short commit hash.
            match run_git(&["rev-parse", "--short", "HEAD"]) {
                Some(hash) => format!("{cargo_version}-dev-{hash}"),
                None => {
                    // git unavailable or not a git repo — use plain version.
                    cargo_version.to_string()
                }
            }
        }
    }
}

/// Run a git sub-command and return trimmed stdout on success.
///
/// Returns `None` if git is not found, the command fails, or output is empty.
fn run_git(args: &[&str]) -> Option<String> {
    let output = Command::new("git").args(args).output().ok()?;

    if !output.status.success() {
        return None;
    }

    let stdout = String::from_utf8(output.stdout).ok()?;
    let trimmed = stdout.trim().to_string();

    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed)
    }
}
