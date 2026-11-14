//! The seed and template and annex pins of obligation PORT3-5,
//! re-expressed from the retired Python setup suite with the same
//! files and the same sentences, the gate sentence standing at the
//! wording obligation PORT3-6 gives it.

use std::path::PathBuf;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn read(rel: &str) -> String {
    std::fs::read_to_string(root().join(rel)).expect("a standing file of the tree reads")
}

#[test]
fn test_seed_files_present() {
    let readme = read("README.md");
    let licence = read("LICENSE");
    let gitignore = read(".gitignore");
    assert!(readme.contains("# tameshi"));
    assert!(readme.contains("The evidence pack tool for checks"));
    assert!(readme.contains("Closed rounds stand at 5 of 100"));
    assert!(readme.contains("## Licence"));
    assert!(licence.contains("MIT License"));
    assert!(gitignore.contains("__pycache__/"));
    assert!(gitignore.contains(".env"));
}

#[test]
fn test_agents_template_and_annex() {
    let agents = read("AGENTS.md");
    let marker = "<!-- repo annex below this line -->";
    assert!(agents.starts_with("# AGENTS.md"));
    assert!(agents.contains(
        "This repo is one of the repos in David Engelmann's 39 repo storefront perfection program"
    ));
    assert!(agents.contains(marker));
    let annex = agents
        .split(marker)
        .nth(1)
        .expect("the annex stands below its marker");
    assert!(annex.contains("Tameshi judges checks, verdictloop judges changes"));
    assert!(annex.contains("at least 30 canaryfield-scored planted defects"));
    assert!(annex.contains("at least 14 days of catchledger CI history"));
    assert!(annex.contains("Rulings stay human signed"));
    assert!(annex.contains("deletes nothing"));
    assert!(annex.contains("synthetic fixtures only and no network in the tool or the tests"));
    assert!(annex.contains("The local gate is cargo test."));
    assert!(annex.contains("closed round count stands in the README and moves only at a closure"));
}
