use clap::Parser;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use dirs::home_dir;
use serde::Deserialize;
use chrono::Datelike;

const CI_TEMPLATE: &str = r#"
name: CI

on:
  push:
    branches: [ "main" ]
  pull_request:

jobs:
  test:
    name: Test on ${{ matrix.os }}
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, macos-latest, windows-latest]

    steps:
      - uses: actions/checkout@v4

      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable

      - name: Build
        run: cargo build --verbose

      - name: Run tests
        run: cargo test --verbose

      - name: Check formatting
        run: cargo fmt -- --check

      - name: Run clippy
        run: cargo clippy -- -D warnings
"#;

/// Cargo wrapper so you can run `cargo setup`
#[derive(Parser)]
#[command(name = "cargo", bin_name = "cargo")]
enum Cargo {
    /// Scaffold a new crate with profile-based extras
    Setup(SetupArgs),
}

#[derive(Parser)]
struct SetupArgs {
    /// Name of the new crate
    name: String,
    /// Create a binary (default is library)
    #[arg(long)]
    bin: bool,
    /// License override (e.g. MIT, Apache-2.0)
    #[arg(long)]
    license: Option<String>,
}

#[derive(Deserialize)]
struct Profile {
    name: Option<String>,
    email: Option<String>,
    github: Option<String>,
    license: Option<String>,
    organization: Option<String>,
}

impl Profile {
    fn path() -> PathBuf {
        home_dir().unwrap().join(".cargo-me.toml")
    }

    fn load() -> Option<Self> {
        let path = Self::path();
        if path.exists() {
            let contents = fs::read_to_string(&path).ok()?;
            toml::from_str(&contents).ok()
        } else {
            None
        }
    }
}

fn main() {
    match Cargo::parse() {
        Cargo::Setup(args) => {
            let profile = Profile::load();
            let license = args
                .license
                .or_else(|| profile.as_ref().and_then(|p| p.license.clone()))
                .unwrap_or_else(|| "MIT".to_string());

            // 1. Run cargo new
            let mut cmd = Command::new("cargo");
            cmd.arg("new").arg(&args.name);
            if args.bin {
                cmd.arg("--bin");
            }
            let status = cmd.status().expect("failed to run cargo new");
            if !status.success() {
                eprintln!("cargo new failed");
                return;
            }

            let crate_path = PathBuf::from(&args.name);

            // 2. Enhance Cargo.toml
            let cargo_toml_path = crate_path.join("Cargo.toml");
            let mut cargo_toml = fs::read_to_string(&cargo_toml_path).unwrap();

            if let Some(profile) = &profile {
                if let Some(author) = &profile.name {
                    let email = profile.email.clone().unwrap_or_default();
                    cargo_toml.push_str(&format!("authors = [\"{} <{}>\"]\n", author, email));
                }
                cargo_toml.push_str(&format!("license = \"{}\"\n", license));
                if let Some(gh) = &profile.github {
                    cargo_toml.push_str(&format!(
                        "repository = \"https://github.com/{}/{}\"\n",
                        gh, args.name
                    ));
                }
            }
            fs::write(&cargo_toml_path, cargo_toml).unwrap();

            // 3. Add README.md with CI badge + install instructions
            let readme_path = crate_path.join("README.md");
            if !readme_path.exists() {
                let gh_user_owned = profile
                    .as_ref()
                    .and_then(|p| p.github.clone())
                    .unwrap_or_else(|| "your-github".to_string());
                let gh_user = &gh_user_owned;

                let ci_badge = format!(
                    "[![CI](https://github.com/{}/{}/actions/workflows/ci.yml/badge.svg)](https://github.com/{}/{}/actions)",
                    gh_user, args.name, gh_user, args.name
                );

                let mut readme = format!("# {}\n\n{}\n\n", args.name, ci_badge);

                if let Some(profile) = &profile {
                    if let Some(gh) = &profile.github {
                        readme.push_str(&format!(
                            "Created by [{}](https://github.com/{})\n\n",
                            gh, gh
                        ));
                    }
                }

                readme.push_str(&format!(
                    "## 📦 Installation\n\n```bash\ncargo install {}\n```\n\n",
                    args.name
                ));

                readme.push_str(
                    "## 🚀 Usage\n\n```rust\nfn main() {\n    println!(\"Hello from your new crate!\");\n}\n```\n",
                );

                fs::write(readme_path, readme).unwrap();
            }

            // 4. Add LICENSE
            let license_path = crate_path.join("LICENSE");
            if !license_path.exists() {
                let year = chrono::Utc::now().year();
                let org = profile
                    .as_ref()
                    .and_then(|p| p.organization.clone())
                    .unwrap_or_else(|| "Your Org".into());

                let license_text = format!(
                    "Copyright (c) {} {}\n\nLicensed under the {} license.",
                    year, org, license
                );
                fs::write(license_path, license_text).unwrap();
            }

            // 5. Add tests/ and benches/
            let test_path = crate_path.join("tests");
            fs::create_dir_all(&test_path).unwrap();
            fs::write(
                test_path.join("basic.rs"),
                "#[test]\nfn it_works() {\n    assert_eq!(2+2, 4);\n}\n",
            )
            .unwrap();

            let bench_path = crate_path.join("benches");
            fs::create_dir_all(&bench_path).unwrap();
            fs::write(
                bench_path.join("bench.rs"),
                "// Basic benchmark (requires criterion)\nfn main() { println!(\"Run with cargo bench\"); }\n",
            )
            .unwrap();

            // 6. Add CHANGELOG.md
            let changelog_path = crate_path.join("CHANGELOG.md");
            if !changelog_path.exists() {
                let changelog = format!(
                    "# Changelog\n\nAll notable changes to `{}` will be documented here.\n\n\
                    The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),\n\
                    and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).\n\n\
                    ## [Unreleased]\n- Initial scaffold\n",
                    args.name
                );
                fs::write(changelog_path, changelog).unwrap();
            }

            // 7. Add GitHub Actions CI workflow
            let ci_path = crate_path.join(".github/workflows");
            fs::create_dir_all(&ci_path).unwrap();
            let ci_file = ci_path.join("ci.yml");
            if !ci_file.exists() {
                fs::write(&ci_file, CI_TEMPLATE).unwrap();
            }

            println!(
                "✅ Scaffolded project `{}` with license `{}` and extras.",
                args.name, license
            );
        }
    }
}
