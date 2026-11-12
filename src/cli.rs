use std::path::PathBuf;

use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Clone, Debug, ValueEnum)]
pub enum Template {
    Executable,
    Library,
    HeaderOnly,
}

#[derive(Clone, Debug, ValueEnum)]
pub enum Standard {
    #[value(name = "c11")]
    C11,
    #[value(name = "c17")]
    C17,
    #[value(name = "c++11", alias = "cxx11")]
    CXX11,
    #[value(name = "c++14", alias = "cxx14")]
    CXX14,
    #[value(name = "c++17", alias = "cxx17")]
    CXX17,
    #[value(name = "c++20", alias = "cxx20")]
    CXX20,
    #[value(name = "c++23", alias = "cxx23")]
    CXX23,
}

#[derive(Debug, Parser)]
#[command(
    name = "cforge",
    version,
    about = "A small, batteries-included CMake project manager"
)]
pub struct Cli {
    #[arg(long, global = true)]
    pub no_color: bool,
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Create a new project directory and initialize it.
    New(NewArgs),
    /// Initialize a project in an existing directory.
    Init(InitArgs),
    /// Configure and compile the project with CMake.
    Build(BuildArgs),
    /// Build and run the project's executable.
    Run(RunArgs),
    /// Remove generated build artifacts.
    Clean(CleanArgs),
    /// List source files and project metadata.
    List(ListArgs),
    /// Check that Forge's required tools are available.
    Doctor,
}

#[derive(Debug, Args)]
pub struct NewArgs {
    pub name: String,
    #[arg(long, short, default_value = "cpp", value_parser = ["c", "cpp"])]
    pub language: String,
    #[arg(long)]
    pub force: bool,
    #[arg(long, value_enum)]
    pub standard: Option<Standard>,
    #[arg(long, value_enum, default_value = "executable")]
    pub template: Template,
    #[arg(long)]
    pub tests: bool,
    #[arg(long)]
    pub git: bool,
    #[arg(long)]
    pub compiler: Option<String>,
}

#[derive(Debug, Args)]
pub struct InitArgs {
    #[arg(default_value = ".")]
    pub path: PathBuf,
    #[arg(long, short)]
    pub force: bool,
    #[arg(long, value_enum)]
    pub standard: Option<Standard>,
    #[arg(long, value_enum, default_value = "executable")]
    pub template: Template,
    #[arg(long)]
    pub tests: bool,
    #[arg(long)]
    pub git: bool,
    #[arg(long)]
    pub compiler: Option<String>,
}

#[derive(Debug, Args)]
pub struct BuildArgs {
    #[arg(long, short, default_value = ".")]
    pub path: PathBuf,
    #[arg(long, short, default_value_t = 0)]
    pub jobs: usize,
    #[arg(long)]
    pub release: bool,
    #[arg(long)]
    pub debug: bool,
    #[arg(long)]
    pub clean: bool,
    #[arg(long)]
    pub generator: Option<String>,
    #[arg(long)]
    pub compiler: Option<String>,
}

#[derive(Debug, Args)]
pub struct RunArgs {
    #[arg(long, short, default_value = ".")]
    pub path: PathBuf,
    #[arg(long)]
    pub release: bool,
    #[arg(last = true)]
    pub args: Vec<String>,
}

#[derive(Debug, Args)]
pub struct CleanArgs {
    #[arg(long, short, default_value = ".")]
    pub path: PathBuf,
    #[arg(long)]
    pub all: bool,
}

#[derive(Debug, Args)]
pub struct ListArgs {
    #[arg(long, short, default_value = ".")]
    pub path: PathBuf,
}
