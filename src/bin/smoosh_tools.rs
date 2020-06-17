use std::collections::HashMap;
use std::env::{self, Args};
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::{self, Command};

macro_rules! try_finally {
    ({$($t: tt)*} {$($f: tt)*}) => {
        let result = (|| -> Result<(), Error> {
            $($t)*
            Ok(())
        })();
        $($f)*
        result?
    }
}

/// Simple wrapper for logging.
///
/// Do not use env_logger etc, to avoid adding extra dependency to library.
/// See https://github.com/rust-lang/rfcs/pull/2887
macro_rules! log_info {
    ($($t: tt)*) => {
        print!("[INFO] ");
        println!($($t)*);
    }
}

#[derive(Debug)]
enum Error {
    Generic(String),
    SubProcessError(String, Option<i32>),
    IO(String, std::io::Error),
    Encode(String, std::str::Utf8Error),
}

impl Error {
    fn dump(&self) {
        match self {
            Error::Generic(message) => {
                println!("{}", message);
            }
            Error::SubProcessError(message, code) => {
                println!("{}", message);
                match code {
                    Some(code) => println!("Subprocess exit with exit status: {}", code),
                    None => println!("Subprocess terminated by signal"),
                }
            }
            Error::IO(message, e) => {
                println!("{}", message);
                println!("{}", e);
            }
            Error::Encode(message, e) => {
                println!("{}", message);
                println!("{}", e);
            }
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum CommandType {
    Build,
    Shell,
    Test,
    Bump,
    Gen,
    Try,
}

#[derive(Debug, Copy, Clone)]
enum BuildType {
    Opt,
    Debug,
}

/// Parse command line arguments.
///
/// Do not use `clap` here, to avoid adding extra dependency to library.
/// See https://github.com/rust-lang/rfcs/pull/2887
#[derive(Debug)]
struct SimpleArgs {
    command: CommandType,
    build_type: BuildType,
    moz_path: String,
    remote: String,
}

impl SimpleArgs {
    fn parse(mut args: Args) -> Self {
        // Skip binary path.
        let _ = args.next().unwrap();

        let command = match args.next() {
            Some(command) => match command.as_str() {
                "build" => CommandType::Build,
                "test" => CommandType::Test,
                "shell" => CommandType::Shell,
                "bump" => CommandType::Bump,
                "gen" => CommandType::Gen,
                "try" => CommandType::Try,
                _ => Self::show_usage(),
            },
            None => Self::show_usage(),
        };

        let mut plain_args = Vec::new();

        let mut remote = "origin".to_string();
        let mut moz_path = Self::guess_moz();
        let mut build_type = BuildType::Debug;

        for arg in args {
            if arg.starts_with("-") {
                if arg.contains("=") {
                    let mut split = arg.split("=");
                    let name = match split.next() {
                        Some(s) => s,
                        None => Self::show_usage(),
                    };
                    let value = match split.next() {
                        Some(s) => s,
                        None => Self::show_usage(),
                    };

                    match name {
                        "--remote" => {
                            remote = value.to_string();
                        }
                        _ => {
                            Self::show_usage();
                        }
                    }
                } else {
                    match arg.as_str() {
                        "--opt" => {
                            build_type = BuildType::Opt;
                        }
                        _ => {
                            Self::show_usage();
                        }
                    }
                }
            } else {
                plain_args.push(arg);
            }
        }

        if !plain_args.is_empty() {
            moz_path = plain_args.remove(0);
        }

        if !plain_args.is_empty() {
            Self::show_usage();
        }

        SimpleArgs {
            command,
            build_type,
            moz_path,
            remote,
        }
    }

    fn show_usage() -> ! {
        println!(
            r#"Tools for jsparagus + SmooshMonkey development

USAGE:
    cargo run --bin smoosh_tools [COMMAND] [OPTIONS]

COMMAND:
    build [--opt] [MOZILLA_CENTRAL]
        Build SpiderMonkey JS shell with SmooshMonkey enabled, using this
        jsparagus clone instead of vendored one
    shell [--opt] [MOZILLA_CENTRAL]
        Run SpiderMonkey JS shell binary built by "build" command
    test [--opt] [MOZILLA_CENTRAL]
        Run jstests/jit-test with SpiderMonkey JS shell binary built by
        "build" command
    bump [MOZILLA_CENTRAL]
        Bump jsparagus version referred by mozilla-central to the latest
        "ci_generated" branch HEAD, and re-vendor jsparagus
    try [--remote=REMOTE] [MOZILLA_CENTRAL]
        Push to try with current jsparagus branch
        This pushes current jsparagus branch to "generated" branch, and
        modifies the reference in mozilla-central to it, and pushes to try
        This requires L1 Commit Access for hg.mozilla.org,
        and mozilla-central should be a Git repository
    gen [--remote=REMOTE]
        Push current jsparagus branch to "generated" branch, with generated
        files included, to refer from mozilla-central

OPTIONS:
    MOZILLA_CENTRAL Path to mozilla-central or mozilla-unified clone
                    This can be omitted if mozilla-central or mozilla-unified
                    is placed next to jsparagus clone directory
    --opt           Use optimized build configuration, instead of debug build
    --remote=REMOTE The name of remote to push the generated branch to
                    Defaults to "origin"
"#
        );
        process::exit(-1)
    }

    fn guess_moz() -> String {
        let cwd = match env::current_dir() {
            Ok(cwd) => cwd,
            _ => return "../mozilla-central".to_string(),
        };

        for path in vec!["../mozilla-central", "../mozilla-unified"] {
            let topsrcdir = Path::new(&cwd).join(path);
            if topsrcdir.exists() {
                return path.to_string();
            }
        }

        return "../mozilla-central".to_string();
    }
}

#[derive(Debug)]
struct MozillaTree {
    topsrcdir: PathBuf,
    smoosh_cargo: PathBuf,
}

impl MozillaTree {
    fn try_new(path: &String) -> Result<Self, Error> {
        let rel_topsrcdir = Path::new(path);
        let cwd = env::current_dir().unwrap();
        let topsrcdir = Path::new(&cwd).join(rel_topsrcdir);
        if !topsrcdir.exists() {
            return Err(Error::Generic(format!(
                "{:?} doesn't exist. Please specify a path to mozilla-central\n
For more information, see https://github.com/mozilla-spidermonkey/jsparagus/wiki/SpiderMonkey",
                topsrcdir
            )));
        }
        let topsrcdir = topsrcdir.canonicalize().unwrap();
        let cargo = topsrcdir
            .join("js")
            .join("src")
            .join("frontend")
            .join("smoosh")
            .join("Cargo.toml");
        if !cargo.exists() {
            return Err(Error::Generic(format!(
                "{:?} doesn't exist. Please specify a path to mozilla-central",
                cargo
            )));
        }

        Ok(Self {
            topsrcdir: topsrcdir.to_path_buf(),
            smoosh_cargo: cargo.to_path_buf(),
        })
    }
}

#[derive(Debug)]
struct JsparagusTree {
    topsrcdir: PathBuf,
    mozconfigs: PathBuf,
}

impl JsparagusTree {
    fn try_new() -> Result<Self, Error> {
        let cwd = env::current_dir().unwrap();
        let topsrcdir = Path::new(&cwd);
        let cargo = topsrcdir.join("Cargo.toml");
        if !cargo.exists() {
            return Err(Error::Generic(format!(
                "{:?} doesn't exist. Please run smoosh_tools in jsparagus top level directory",
                cargo
            )));
        }

        let mozconfigs = topsrcdir.join("mozconfigs");
        if !mozconfigs.exists() {
            return Err(Error::Generic(format!(
                "{:?} doesn't exist. Please run smoosh_tools in jsparagus top level directory",
                mozconfigs
            )));
        }

        Ok(Self {
            topsrcdir: topsrcdir.to_path_buf(),
            mozconfigs: mozconfigs.to_path_buf(),
        })
    }

    fn mozconfig(&self, build_type: BuildType) -> PathBuf {
        self.mozconfigs.join(match build_type {
            BuildType::Opt => "smoosh-opt",
            BuildType::Debug => "smoosh-debug",
        })
    }
}

/// Run `command`, and check if the exit code is successful.
/// Returns Err if failed to run the command, or the exit code is non-zero.
fn check_command(command: &mut Command) -> Result<(), Error> {
    log_info!("$ {:?}", command);
    let status = command
        .status()
        .map_err(|e| Error::IO(format!("Failed to run {:?}", command), e))?;
    if !status.success() {
        return Err(Error::SubProcessError(
            format!("Failed to run {:?}", command),
            status.code(),
        ));
    }

    Ok(())
}

/// Run `command`, and returns its status code.
/// Returns Err if failed to run the command, or the subprocess is terminated
/// by signal.
fn get_retcode(command: &mut Command) -> Result<i32, Error> {
    log_info!("$ {:?}", command);
    let status = command
        .status()
        .map_err(|e| Error::IO(format!("Failed to run {:?}", command), e))?;
    if !status.success() {
        match status.code() {
            Some(code) => return Ok(code),
            None => {
                return Err(Error::SubProcessError(
                    format!("Failed to run {:?}", command),
                    None,
                ))
            }
        }
    }

    Ok(0)
}

/// Run `command`, and returns its stdout
/// Returns Err if failed to run the command.
fn get_output(command: &mut Command) -> Result<String, Error> {
    log_info!("$ {:?}", command);
    let output = command
        .output()
        .map_err(|e| Error::IO(format!("Failed to run {:?}", command), e))?;
    let stdout = std::str::from_utf8(output.stdout.as_slice())
        .map_err(|e| Error::Encode(format!("Failed to decode the output of {:?}", command), e))?
        .to_string();
    Ok(stdout)
}

struct GitRepository {
    topsrcdir: PathBuf,
}

impl GitRepository {
    fn try_new(topsrcdir: PathBuf) -> Result<Self, Error> {
        if !topsrcdir.join(".git").as_path().exists() {
            return Err(Error::Generic(format!(
                "{:?} is not Git repository",
                topsrcdir
            )));
        }

        Ok(Self { topsrcdir })
    }

    fn run(&self, args: &[&str]) -> Result<(), Error> {
        check_command(
            Command::new("git")
                .args(args)
                .current_dir(self.topsrcdir.clone()),
        )
    }

    fn get_retcode(&self, args: &[&str]) -> Result<i32, Error> {
        get_retcode(
            Command::new("git")
                .args(args)
                .current_dir(self.topsrcdir.clone()),
        )
    }

    fn get_output(&self, args: &[&str]) -> Result<String, Error> {
        get_output(
            Command::new("git")
                .args(args)
                .current_dir(self.topsrcdir.clone()),
        )
    }

    /// Checks if there's no uncommitted changes.
    fn assert_clean(&self) -> Result<(), Error> {
        log_info!("Checking {} is clean", self.topsrcdir.to_str().unwrap());
        let code = self.get_retcode(&["diff-index", "--quiet", "HEAD", "--"])?;
        if code != 0 {
            return Err(Error::Generic(format!(
                "Uncommitted changes found in {}",
                self.topsrcdir.to_str().unwrap()
            )));
        }

        let code = self.get_retcode(&["diff-index", "--cached", "--quiet", "HEAD", "--"])?;
        if code != 0 {
            return Err(Error::Generic(format!(
                "Uncommitted changes found in {}",
                self.topsrcdir.to_str().unwrap()
            )));
        }

        Ok(())
    }

    /// Returns the current branch, or "HEAD" if it's detached head..
    fn branch(&self) -> Result<String, Error> {
        Ok(self
            .get_output(&["rev-parse", "--abbrev-ref", "HEAD"])?
            .trim()
            .to_string())
    }

    /// Ensure a remote with `name` exists.
    /// If it doesn't exist, add remote with `name` and `url`.
    fn ensure_remote(&self, name: &'static str, url: &'static str) -> Result<(), Error> {
        for line in self.get_output(&["remote"])?.split("\n") {
            if line == name {
                return Ok(());
            }
        }

        self.run(&["remote", "add", name, url])?;

        Ok(())
    }

    /// Returns a map of remote branches.
    fn ls_remote(&self, remote: &'static str) -> Result<HashMap<String, String>, Error> {
        let mut map = HashMap::new();
        for line in self.get_output(&["ls-remote", remote])?.split("\n") {
            let mut split = line.split("\t");
            let sha = match split.next() {
                Some(s) => s,
                None => continue,
            };
            let ref_name = match split.next() {
                Some(s) => s,
                None => continue,
            };
            map.insert(ref_name.to_string(), sha.to_string());
        }

        Ok(map)
    }
}

/// Trait for replacing dependencies in Cargo.toml.
trait DependencyLineReplacer {
    /// Receives `line` for official jsparagus reference,
    /// and adds modified jsparagus reference to `lines`.
    fn on_official(&self, line: &str, lines: &mut Vec<String>);
}

/// Replace jsparagus reference to `sha` in official ci_generated branch.
struct OfficialDependencyLineReplacer {
    sha: String,
}

impl DependencyLineReplacer for OfficialDependencyLineReplacer {
    fn on_official(&self, _line: &str, lines: &mut Vec<String>) {
        let newline = format!("jsparagus = {{ git = \"https://github.com/mozilla-spidermonkey/jsparagus\", rev = \"{}\" }}", self.sha);
        log_info!("Rewriting jsparagus reference: {}", newline);
        lines.push(newline);
    }
}

/// Replace jsparagus reference to local clone.
struct LocalDependencyLineReplacer {
    jsparagus: PathBuf,
}

impl DependencyLineReplacer for LocalDependencyLineReplacer {
    fn on_official(&self, line: &str, lines: &mut Vec<String>) {
        lines.push(format!("# {}", line));
        let newline = format!(
            "jsparagus = {{ path = \"{}\" }}",
            self.jsparagus.to_str().unwrap()
        );
        log_info!("Rewriting jsparagus reference: {}", newline);
        lines.push(newline);
    }
}

/// Replace jsparagus reference to a remote branch in forked repository.
struct ForkDependencyLineReplacer {
    github_user: String,
    branch: String,
}

impl DependencyLineReplacer for ForkDependencyLineReplacer {
    fn on_official(&self, line: &str, lines: &mut Vec<String>) {
        lines.push(format!("# {}", line));
        let newline = format!(
            "jsparagus = {{ git = \"https://github.com/{}/jsparagus\", branch = \"{}\" }}",
            self.github_user, self.branch
        );
        log_info!("Rewriting jsparagus reference: {}", newline);
        lines.push(newline);
    }
}

fn read_file(path: &PathBuf) -> Result<String, Error> {
    let mut file = File::open(path.as_path())
        .map_err(|e| Error::IO(format!("Couldn't open {}", path.to_str().unwrap()), e))?;
    let mut content = String::new();
    file.read_to_string(&mut content)
        .map_err(|e| Error::IO(format!("Couldn't read {}", path.to_str().unwrap()), e))?;

    Ok(content)
}

fn write_file(path: &PathBuf, content: String) -> Result<(), Error> {
    let mut file = File::create(path.as_path()).map_err(|e| {
        Error::IO(
            format!("Couldn't open {} in write mode", path.to_str().unwrap()),
            e,
        )
    })?;
    file.write_all(content.as_bytes())
        .map_err(|e| Error::IO(format!("Couldn't write {}", path.to_str().unwrap()), e))?;

    Ok(())
}

fn update_cargo<T>(cargo: &PathBuf, replacer: T) -> Result<(), Error>
where
    T: DependencyLineReplacer,
{
    let content = read_file(cargo)?;
    let mut filtered_lines = Vec::new();
    for line in content.split("\n") {
        if line.starts_with(
            "# jsparagus = { git = \"https://github.com/mozilla-spidermonkey/jsparagus\",",
        ) || line.starts_with(
            "jsparagus = { git = \"https://github.com/mozilla-spidermonkey/jsparagus\",",
        ) {
            let orig_line = if line.starts_with("# ") {
                &line[2..]
            } else {
                line
            };
            replacer.on_official(orig_line, &mut filtered_lines)
        } else if line.starts_with("jsparagus = ") {
        } else {
            filtered_lines.push(line.to_string());
        }
    }
    write_file(cargo, filtered_lines.join("\n"))
}

fn run_mach(command: &'static str, args: &SimpleArgs) -> Result<(), Error> {
    let moz = MozillaTree::try_new(&args.moz_path)?;
    let jsparagus = JsparagusTree::try_new()?;
    let mozconfig = jsparagus.mozconfig(args.build_type);

    check_command(
        Command::new(moz.topsrcdir.join("mach").to_str().unwrap())
            .arg(command)
            .current_dir(moz.topsrcdir)
            .env("MOZCONFIG", mozconfig.to_str().unwrap()),
    )
}

fn build(args: &SimpleArgs) -> Result<(), Error> {
    let moz = MozillaTree::try_new(&args.moz_path)?;
    let jsparagus = JsparagusTree::try_new()?;

    update_cargo(
        &moz.smoosh_cargo,
        LocalDependencyLineReplacer {
            jsparagus: jsparagus.topsrcdir,
        },
    )?;

    run_mach("build", args)
}

fn shell(args: &SimpleArgs) -> Result<(), Error> {
    run_mach("run", args)
}

fn test(args: &SimpleArgs) -> Result<(), Error> {
    run_mach("jstests", args)?;
    run_mach("jit-test", args)
}

fn vendor(moz: &MozillaTree) -> Result<(), Error> {
    check_command(
        Command::new(moz.topsrcdir.join("mach").to_str().unwrap())
            .arg("vendor")
            .arg("rust")
            .current_dir(moz.topsrcdir.clone()),
    )
}

fn bump(args: &SimpleArgs) -> Result<(), Error> {
    let moz = MozillaTree::try_new(&args.moz_path)?;
    let jsparagus = JsparagusTree::try_new()?;

    let jsparagus_repo = GitRepository::try_new(jsparagus.topsrcdir.clone())?;

    log_info!("Checking ci_generated branch HEAD");

    let remotes =
        jsparagus_repo.ls_remote("https://github.com/mozilla-spidermonkey/jsparagus.git")?;

    let branch = "refs/heads/ci_generated";

    let ci_generated_sha = match remotes.get(branch) {
        Some(sha) => sha,
        None => {
            return Err(Error::Generic(format!("{} not found in upstream", branch)));
        }
    };

    log_info!("ci_generated branch HEAD = {}", ci_generated_sha);

    update_cargo(
        &moz.smoosh_cargo,
        OfficialDependencyLineReplacer {
            sha: ci_generated_sha.clone(),
        },
    )?;

    vendor(&moz)?;

    log_info!("Please add updated files and commit them.");

    Ok(())
}

/// Parse remote string and get GitHub username.
/// Currently this supports only SSH format.
fn parse_github_username(remote: String) -> Result<String, Error> {
    let git_prefix = "git@github.com:";
    let git_suffix = "/jsparagus.git";

    if remote.starts_with(git_prefix) && remote.ends_with(git_suffix) {
        return Ok(remote.replace(git_prefix, "").replace(git_suffix, ""));
    }

    Err(Error::Generic(format!(
        "Failed to get GitHub username: {}",
        remote
    )))
}

struct BranchInfo {
    github_user: String,
    branch: String,
}

/// Create "generated" branch and push to remote, and returns
/// GitHub username and branch name.
fn push_to_gen_branch(args: &SimpleArgs) -> Result<BranchInfo, Error> {
    let jsparagus = JsparagusTree::try_new()?;

    let jsparagus_repo = GitRepository::try_new(jsparagus.topsrcdir.clone())?;
    jsparagus_repo.assert_clean()?;

    log_info!("Getting GitHub username and current branch");

    let origin = jsparagus_repo
        .get_output(&["remote", "get-url", args.remote.as_str()])?
        .trim()
        .to_string();
    let github_user = parse_github_username(origin)?;

    let branch = jsparagus_repo.branch()?;
    if branch == "HEAD" {
        return Err(Error::Generic(format!(
            "Detached HEAD is not supported. Please checkout a branch"
        )));
    }

    let gen_branch = format!("{}-generated-branch", branch);

    log_info!("Creating {} branch", gen_branch);

    jsparagus_repo.run(&["checkout", "-b", gen_branch.as_str()])?;

    try_finally!({
        log_info!("Updating generated files");

        check_command(
            Command::new("make")
                .arg("all")
                .current_dir(jsparagus.topsrcdir.clone()),
        )?;

        log_info!("Committing generated files");

        jsparagus_repo.run(&["add", "--force", "*_generated.rs"])?;

        try_finally!({
            jsparagus_repo.run(&["commit", "-m", "Add generated files"])?;

            try_finally!({
                log_info!("Pushing to {}", gen_branch);
                jsparagus_repo.run(&["push", "-f", args.remote.as_str(), gen_branch.as_str()])?;
            } {
                // Revert the commit, wihtout removing *_generated.rs.
                jsparagus_repo.run(&["reset", "--soft", "HEAD^"])?;
            });
        } {
            jsparagus_repo.run(&["reset"])?;
        });
    } {
        jsparagus_repo.run(&["checkout", branch.as_str()])?;
        jsparagus_repo.run(&["branch", "-D", gen_branch.as_str()])?;
    });

    Ok(BranchInfo {
        github_user,
        branch: gen_branch,
    })
}

fn gen_branch(args: &SimpleArgs) -> Result<(), Error> {
    push_to_gen_branch(args)?;

    Ok(())
}

fn push_try(args: &SimpleArgs) -> Result<(), Error> {
    let moz = MozillaTree::try_new(&args.moz_path)?;

    let moz_repo = GitRepository::try_new(moz.topsrcdir.clone())?;
    moz_repo.assert_clean()?;

    let branch_info = push_to_gen_branch(args)?;

    moz_repo.ensure_remote("try", "hg::https://hg.mozilla.org/try")?;

    update_cargo(
        &moz.smoosh_cargo,
        ForkDependencyLineReplacer {
            github_user: branch_info.github_user,
            branch: branch_info.branch,
        },
    )?;

    vendor(&moz)?;

    moz_repo.run(&["add", "."])?;
    moz_repo.run(&["commit", "-m", "Update vendored crates for jsparagus"])?;
    try_finally!({
        let syntax = "try: -b do -p sm-smoosh-linux64,sm-nonunified-linux64 -u none -t none";
        moz_repo.run(&["commit", "--allow-empty", "-m", syntax])?;
        try_finally!({
            moz_repo.run(&["push", "try"])?;
        } {
            moz_repo.run(&["reset", "--hard", "HEAD^"])?;
        });
    } {
        moz_repo.run(&["reset", "--hard", "HEAD^"])?;
    });

    Ok(())
}

fn main() {
    let args = SimpleArgs::parse(env::args());

    let result = match args.command {
        CommandType::Build => build(&args),
        CommandType::Shell => shell(&args),
        CommandType::Test => test(&args),
        CommandType::Bump => bump(&args),
        CommandType::Gen => gen_branch(&args),
        CommandType::Try => push_try(&args),
    };

    match result {
        Ok(_) => {}
        Err(e) => e.dump(),
    }
}
