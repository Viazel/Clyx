use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result, bail};
use colored::Colorize;
use dialoguer::Confirm;
use indicatif::{ProgressBar, ProgressStyle};

use crate::{
    cli::{BuildArgs, CleanArgs, Command, InitArgs, ListArgs, NewArgs, RunArgs},
    config::{CONFIG_FILE, ProjectConfig},
    process, templates,
};

pub fn execute(cli: crate::cli::Cli) -> Result<()> {
    colored::control::set_override(!cli.no_color);
    match cli.command {
        Command::New(a) => new(a),
        Command::Init(a) => init(a),
        Command::Build(a) => build(a),
        Command::Run(a) => run(a),
        Command::Clean(a) => clean(a),
        Command::List(a) => list(a),
        Command::Doctor => doctor(),
    }
}

fn new(args: NewArgs) -> Result<()> {
    let root = PathBuf::from(&args.name);
    let name = root
        .file_name()
        .and_then(|n| n.to_str())
        .filter(|n| !n.is_empty())
        .unwrap_or("forge-project")
        .to_owned();
    templates::validate_name(&name)?;
    let mut cfg = ProjectConfig::new(name, args.language);
    cfg.defaults.standard = templates::standard_string(&args.standard, &cfg.language);
    cfg.defaults.template = format!("{:?}", args.template).to_lowercase();
    cfg.defaults.tests = args.tests;
    cfg.compiler = args.compiler;
    templates::create_project(&root, &cfg, &args.template, args.tests, args.force)?;
    if args.git {
        initialize_git(&root)?;
    }
    println!("{} {}", "Created".green(), root.display());
    Ok(())
}

fn init(args: InitArgs) -> Result<()> {
    let root = fs::canonicalize(&args.path)
        .with_context(|| format!("locating {}", args.path.display()))?;
    let name = root
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("forge-project")
        .to_owned();
    let config = if root.join(CONFIG_FILE).exists() {
        ProjectConfig::load(&root)?
    } else {
        ProjectConfig::new(name, "cpp")
    };
    let mut config = config;
    if args.standard.is_some() {
        config.defaults.standard = templates::standard_string(&args.standard, &config.language);
    }
    if let Some(compiler) = args.compiler {
        config.compiler = Some(compiler);
    }
    config.defaults.template = format!("{:?}", args.template).to_lowercase();
    config.defaults.tests = args.tests;
    templates::validate_name(&config.name)?;
    templates::create_project(&root, &config, &args.template, args.tests, args.force)?;
    if args.git {
        initialize_git(&root)?;
    }
    println!("{} {}", "Initialized".green(), root.display());
    Ok(())
}

fn config(path: &Path) -> Result<(PathBuf, ProjectConfig)> {
    let root = fs::canonicalize(path).with_context(|| format!("locating {}", path.display()))?;
    Ok((root.clone(), ProjectConfig::load(&root)?))
}

fn build(args: BuildArgs) -> Result<()> {
    let (root, cfg) = config(&args.path)?;
    if args.clean && cfg.build_path(&root).exists() {
        fs::remove_dir_all(cfg.build_path(&root))?;
    }
    fs::create_dir_all(cfg.build_path(&root))?;
    let build_dir = cfg.build_path(&root);
    let mut configure = vec![
        "-S".to_owned(),
        ".".to_owned(),
        "-B".to_owned(),
        cfg.build_dir.clone(),
    ];
    if args.release || args.debug || cfg.build.build_type != "Debug" {
        let mode = if args.release {
            "Release"
        } else if args.debug {
            "Debug"
        } else {
            &cfg.build.build_type
        };
        configure.push(format!("-DCMAKE_BUILD_TYPE={mode}"));
    }
    if let Some(generator) = args
        .generator
        .or_else(|| (cfg.build.generator != "default").then(|| cfg.build.generator.clone()))
    {
        configure.extend(["-G".to_owned(), generator]);
    }
    if let Some(compiler) = args.compiler.or(cfg.compiler.clone()) {
        let variable = if cfg.language.eq_ignore_ascii_case("c") {
            "CMAKE_C_COMPILER"
        } else {
            "CMAKE_CXX_COMPILER"
        };
        configure.extend([format!("-D{variable}={compiler}")]);
    }
    process::run("cmake", &configure, &root)?;
    let mut build_args = vec!["--build".to_owned(), cfg.build_dir.clone()];
    if args.jobs > 0 {
        build_args.extend(["--parallel".to_owned(), args.jobs.to_string()]);
    }
    let bar = ProgressBar::new_spinner();
    bar.set_style(ProgressStyle::with_template("{spinner} {msg}")?);
    bar.set_message(format!("Building {}", cfg.name));
    process::run("cmake", &build_args, &root)?;
    bar.finish_with_message("Build complete".green().to_string());
    let _ = build_dir;
    Ok(())
}

fn run(args: RunArgs) -> Result<()> {
    let (root, cfg) = config(&args.path)?;
    build(BuildArgs {
        path: args.path.clone(),
        jobs: 0,
        release: args.release,
        debug: false,
        clean: false,
        generator: None,
        compiler: None,
    })?;
    let executable = root.join(&cfg.build_dir).join(&cfg.name);
    let executable = if executable.exists() {
        executable
    } else {
        #[cfg(windows)]
        {
            root.join(&cfg.build_dir).join(format!("{}.exe", cfg.name))
        }
        #[cfg(not(windows))]
        {
            bail!("built executable not found at {}", executable.display())
        }
    };
    let status = std::process::Command::new(&executable)
        .args(&args.args)
        .current_dir(&root)
        .status()
        .with_context(|| format!("running {}", executable.display()))?;
    if !status.success() {
        bail!("program exited unsuccessfully");
    }
    Ok(())
}

fn clean(args: CleanArgs) -> Result<()> {
    let (root, cfg) = config(&args.path)?;
    let build = cfg.build_path(&root);
    if args.all {
        for entry in fs::read_dir(&root)? {
            let path = entry?.path();
            if path
                .file_name()
                .is_some_and(|n| n == "build" || n == "target")
                && path.is_dir()
            {
                fs::remove_dir_all(path)?;
            }
        }
        println!("{}", "All build artifacts removed".green());
    } else if build.exists() {
        if Confirm::new()
            .with_prompt(format!("Remove {}?", build.display()))
            .default(true)
            .interact()?
        {
            fs::remove_dir_all(&build)?;
            println!("{}", "Build artifacts removed".green());
        }
    } else {
        println!("Nothing to clean");
    }
    Ok(())
}

fn list(args: ListArgs) -> Result<()> {
    let (_, cfg) = config(&args.path)?;
    println!("{} ({})", cfg.name.bold(), cfg.language);
    println!("source: {}", cfg.source);
    println!("build: {}", cfg.build_dir);
    println!("templates: executable, library, header-only");
    Ok(())
}

fn doctor() -> Result<()> {
    println!("Forge {}", env!("CARGO_PKG_VERSION").bold());
    for tool in ["cmake", "cargo", "ninja", "git", "c++"] {
        if process::exists(tool) {
            println!("{} {}", "✓".green(), tool);
        } else {
            println!("{} {}", "✗".red(), tool);
        }
    }
    if !process::exists("cmake") {
        bail!("cmake is required but was not found");
    }

    Ok(())
}

fn initialize_git(root: &Path) -> Result<()> {
    if !process::exists("git") {
        bail!("--git requested but git was not found");
    }
    process::run("git", &["init".to_owned()], root)
}
