# Forge

Forge is a small, cross-platform Rust CLI for creating and managing modern
C and C++ projects built with CMake. Generated projects use target-based CMake
and keep source files, build artifacts, and configuration separate.

## Features

- C11/C17 and C++11/14/17/20/23 executable, static-library, and header-only templates.
- Non-destructive `new` and `init` commands, optional tests and Git initialization.
- CMake configuration/build/run/clean commands with Debug/Release, generator,
  compiler, and parallel-job options.
- Tool diagnostics through `Clyx doctor` and a portable `--no-color` mode.

## Installation and prerequisites

Install Forge locally with Cargo from a cloned checkout:

```sh
git clone https://github.com/your-org/forge.git
cd forge
cargo install --path .
```

The installed `Clyx` executable is then available from Cargo's binary
directory (usually `~/.cargo/bin`):

```sh
Clyx --version
```

Forge itself needs a stable Rust toolchain. Generated projects need CMake 3.16+
and a C or C++ compiler. Ninja and Git are optional unless selected explicitly.

## Quick start

```sh
Clyx new hello --language cpp --standard c++20 --tests --git
cd hello
Clyx build --debug --jobs 8
Clyx run
Clyx build --release
Clyx clean --all
```

Use `Clyx init` in an existing directory. It never replaces existing source
files or `CMakeLists.txt`; `--force` only permits writing missing project
scaffolding in a non-empty directory.

## Commands

| Command | Purpose |
| --- | --- |
| `new NAME` | Create a project (`--language`, `--standard`, `--template`, `--tests`, `--git`, `--compiler`) |
| `init [PATH]` | Initialize an existing directory |
| `build` | Configure and build (`--debug`, `--release`, `--jobs`, `--generator`, `--clean`) |
| `run` | Build and execute the generated executable |
| `clean` | Remove build artifacts; `--all` also removes nested `target`/`build` directories |
| `list` | Show project metadata and available templates |
| `doctor` | Check CMake, compilers, Ninja, and Git |

Run `Clyx --help` or `Clyx COMMAND --help` for complete option details.

## Configuration

Each project has an optional `forge.toml`:

```toml
[defaults]
standard = "c++20"
template = "executable"
tests = true

[build]
generator = "Ninja"
build_type = "Debug"

[ui]
color = true
```

Command-line options override project settings. Configuration is not required
for CMake projects created outside Forge.

## Generated layout

An executable contains `CMakeLists.txt`, `forge.toml`, `README.md`-compatible
source layout (`src/`, optional `tests/`, and `include/`), and a `.gitignore`.
Libraries use `add_library`; header-only libraries use an `INTERFACE` target
and install-ready include paths.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for development and template guidance.

## License

MIT; see [LICENSE](LICENSE).
