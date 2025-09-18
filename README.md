# cargo-setup

A Cargo subcommand that scaffolds new crates with extra polish.  
Think of it as `cargo new` but with **README.md**, **LICENSE**, **CHANGELOG.md**, **tests/**, **benches/**, and metadata auto-filled from your [cargo-me](https://crates.io/crates/cargo-me) profile. It even sets up **CI with GitHub Actions**.

---

## ✨ Features
- Wraps `cargo new` internally — no need to run it separately.
- Auto-fills `authors`, `license`, and `repository` in `Cargo.toml` from your `cargo-me` profile.
- Adds `README.md` with CI badge, author info, installation, and usage example.
- Adds `LICENSE` file with year + organization from profile.
- Creates `tests/basic.rs` and `benches/bench.rs` folders.
- Adds a `CHANGELOG.md` following [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).
- Sets up GitHub Actions CI (`.github/workflows/ci.yml`) for build, test, fmt, clippy on Linux/macOS/Windows.

---

## 📦 Installation

```bash
cargo install cargo-setup
```

Make sure you also have [`cargo-me`](https://crates.io/crates/cargo-me) installed and initialized, since `cargo-setup` uses the profile stored in `~/.cargo-me.toml`.

```bash
cargo install cargo-me
cargo me init
```

---

## 🚀 Usage

### Create a new binary crate
```bash
cargo setup myapp --bin
```

### Create a new library crate
```bash
cargo setup mylib
```

### Override license
```bash
cargo setup mycrate --license Apache-2.0
```

---

## 📊 Example workflow

1. Configure your profile once with `cargo-me`:
   ```bash
   cargo me init
   cargo me set name "JD Plumbing"
   cargo me set email "jdplumbingsoflo@gmail.com"
   cargo me set github "JDPlumbing"
   cargo me set license "MIT"
   ```

2. Scaffold new crates with extras in one command:
   ```bash
   cargo setup shortid-rs --bin
   ```

3. Resulting project structure:
   ```
   shortid-rs/
   ├── Cargo.toml   # with authors/license/repo already filled
   ├── src/main.rs
   ├── README.md    # with CI badge, install, usage
   ├── LICENSE
   ├── CHANGELOG.md
   ├── tests/basic.rs
   ├── benches/bench.rs
   └── .github/workflows/ci.yml
   ```

---

## ⚖️ License

MIT License. See [LICENSE](LICENSE) for details.
