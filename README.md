# cargo-set

A Cargo subcommand that scaffolds new crates with extra polish.  
Think of it as `cargo new` but with **README.md**, **LICENSE**, **tests/**, **benches/**, and metadata auto-filled from your [cargo-me](https://crates.io/crates/cargo-me) profile.

---

## ✨ Features
- Wraps `cargo new` internally — no need to run it separately.
- Auto-fills `authors`, `license`, and `repository` in `Cargo.toml` from your `cargo-me` profile.
- Adds `README.md` with repo/author info.
- Adds `LICENSE` file with year + organization from profile.
- Creates `tests/basic.rs` and `benches/bench.rs` folders.
- Supports `--bin` or `--lib` (just like `cargo new`).

---

## 📦 Installation

```bash
cargo install cargo-set
```

Make sure you also have [`cargo-me`](https://crates.io/crates/cargo-me) installed and initialized, since `cargo-set` uses the profile stored in `~/.cargo-me.toml`.

```bash
cargo install cargo-me
cargo me init
```

---

## 🚀 Usage

### Create a new binary crate
```bash
cargo set myapp --bin
```

### Create a new library crate
```bash
cargo set mylib
```

### Override license
```bash
cargo set mycrate --license Apache-2.0
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
   cargo set shortid-rs --bin
   ```

3. Resulting project structure:
   ```
   shortid-rs/
   ├── Cargo.toml   # with authors/license/repo already filled
   ├── src/main.rs
   ├── README.md
   ├── LICENSE
   ├── tests/basic.rs
   └── benches/bench.rs
   ```

---

## ⚖️ License

MIT License. See [LICENSE](LICENSE) for details.
