use clap::{Parser};
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use dirs::home_dir;
use serde::Deserialize;
use chrono::Datelike;

/// Cargo wrapper so you can run `cargo set`
#[derive(Parser)]
#[command(name = "cargo", bin_name = "cargo")]
struct Cargo {
    /// Create a new crate
    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand)]
enum Commands {
    /// Scaffold a new crate with profile-based extras
    Set {
        /// Name of the new crate
        name: String,
        /// Create a binary (default is library)
        #[arg(long)]
        bin: bool,
        /// License override (e.g. MIT, Apache-2.0)
        #[arg(long)]
        license: Option<String>,
    },
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
    let args = Cargo::parse();

    match args.command {
        Commands::Set { name, bin, license } => {
            let profile = Profile::load();
            let license = license
                .or_else(|| profile.as_ref().and_then(|p| p.license.clone()))
                .unwrap_or_else(|| "MIT".to_string());

            // 1. Run cargo new
            let mut cmd = Command::new("cargo");
            cmd.arg("new").arg(&name);
            if bin {
                cmd.arg("--bin");
            }
            let status = cmd.status().expect("failed to run cargo new");
            if !status.success() {
                eprintln!("cargo new failed");
                return;
            }

            let crate_path = PathBuf::from(&name);

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
                    cargo_toml.push_str(&format!("repository = \"https://github.com/{}/{}\"\n", gh, name));
                }
            }
            fs::write(&cargo_toml_path, cargo_toml).unwrap();

            // 3. Add README.md
            let readme_path = crate_path.join("README.md");
            if !readme_path.exists() {
                let mut readme = format!("# {}\n\n", name);
                if let Some(profile) = &profile {
                    if let Some(gh) = &profile.github {
                        readme.push_str(&format!("Created by [{}](https://github.com/{})\n", gh, gh));
                    }
                }
                fs::write(readme_path, readme).unwrap();
            }

            // 4. Add LICENSE
            let license_path = crate_path.join("LICENSE");
            if !license_path.exists() {
                let year = chrono::Utc::now().year();
                let org = profile.as_ref()
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
            fs::write(test_path.join("basic.rs"),
                "#[test]\nfn it_works() {\n    assert_eq!(2+2, 4);\n}\n"
            ).unwrap();

            let bench_path = crate_path.join("benches");
            fs::create_dir_all(&bench_path).unwrap();
            fs::write(bench_path.join("bench.rs"),
                "// Basic benchmark (requires criterion)\nfn main() { println!(\"Run with cargo bench\"); }\n"
            ).unwrap();

            println!("✅ Scaffolded project `{}` with license `{}` and extras.", name, license);
        }
    }
}
