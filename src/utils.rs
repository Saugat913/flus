use anyhow::Context;

use crate::{app_error, error::Result};
use std::{
    env,
    fs::{self, File},
    io::{self, Write},
    path::{Path, PathBuf},
    process::{Command, Stdio},
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    thread,
    time::Duration,
};

#[derive(Debug)]
pub struct ExecuterConfig {
    pub base_path: PathBuf,
    stdin: Stdio,
    stdout: Stdio,
    stderr: Stdio,
}

impl Default for ExecuterConfig {
    fn default() -> Self {
        Self {
            base_path: env::current_dir().unwrap(),
            stdin: Stdio::piped(),
            stdout: Stdio::piped(),
            stderr: Stdio::piped(),
        }
    }
}

pub fn execute_command(command: &str, args: &[&str], config: ExecuterConfig) -> Result<()> {
    let output = Command::new(command)
        .args(args)
        .current_dir(config.base_path)
        .stdin(config.stdin)
        .stderr(config.stderr)
        .stdout(config.stdout)
        .output();
    match output {
        Err(e) => {
            return Err(app_error!(
                "While executing command {}: {}",
                command,
                e.to_string()
            ));
        }
        Ok(output) => {
            if !output.status.success() {
                let error_msg = String::from_utf8_lossy(&output.stderr);
                let out_msg = String::from_utf8_lossy(&output.stdout);

                return Err(app_error!(
                    "Command '{}' failed.{}{}",
                    command,
                    error_msg.trim(),
                    out_msg.trim()
                ));
            }
        }
    }
    Ok(())
}

pub fn run_with_spinner<T, F>(loading_msg: &str, funtion: F) -> T
where
    F: FnOnce() -> T,
{
    let is_running = Arc::new(AtomicBool::new(true));
    let spinner_running = Arc::clone(&is_running);
    let loading_msg = loading_msg.to_string();
    let spinner_handle = thread::spawn(move || {
        let frames = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧"];
        let mut i = 0;
        while spinner_running.load(Ordering::SeqCst) {
            print!("\r{} {}", frames[i % frames.len()], loading_msg);
            io::stdout().flush().unwrap();
            i += 1;
            thread::sleep(Duration::from_millis(100));
        }
        print!("\r{: <50}\r", "");
        let _ = io::stdout().flush();
    });

    let result = funtion();
    is_running.store(false, Ordering::SeqCst);
    spinner_handle.join().unwrap();
    return result;
}

pub enum FSAction {
    CreateDir {
        path: PathBuf,
    },
    CreateFile {
        path: PathBuf,
        content: Option<String>,
    },
    RemoveDir {
        path: PathBuf,
        recursive: bool,
    },
    RemoveFile {
        path: PathBuf,
    },
}
impl FSAction {
    pub fn create_dir(path: &str) -> Self {
        Self::CreateDir {
            path: PathBuf::from(path),
        }
    }

    pub fn create_file(path: &str, content: Option<&str>) -> Self {
        Self::CreateFile {
            path: PathBuf::from(path),
            content: content.map(Into::into),
        }
    }

    pub fn remove_file(path: &str) -> Self {
        Self::RemoveFile {
            path: PathBuf::from(path),
        }
    }

    pub fn execute(&self, base_path: &Path) -> Result<()> {
        match self {
            FSAction::CreateDir { path } => {
                let full_path = base_path.join(path);
                fs::create_dir_all(&full_path)
                    .map_err(|e| app_error!("Failed to create directory {:?}: {}", full_path, e))?;
            }
            FSAction::CreateFile { path, content } => {
                let full_path = base_path.join(path);
                if let Some(parent) = full_path.parent() {
                    fs::create_dir_all(parent)?;
                }
                let mut f = File::create(&full_path)
                    .map_err(|e| app_error!("Failed to create file {:?}: {}", full_path, e))?;
                if let Some(c) = content {
                    f.write_all(c.as_bytes())?;
                }
            }
            FSAction::RemoveFile { path } => {
                let full_path = base_path.join(path);
                if full_path.exists() {
                    fs::remove_file(&full_path)?;
                }
            }
            FSAction::RemoveDir { path, recursive } => {
                let full_path = base_path.join(path);
                if full_path.exists() {
                    if *recursive {
                        fs::remove_dir_all(&full_path)?;
                    } else {
                        fs::remove_dir(&full_path)?;
                    }
                }
            }
        }
        Ok(())
    }
}


pub fn inject_git_dependency(
    base_path: &std::path::Path,
    package_name: &str,
    git_url: &str,
) -> anyhow::Result<()> {
    anyhow::ensure!(!package_name.is_empty(), "package_name must not be empty");
    anyhow::ensure!(!git_url.is_empty(), "git_url must not be empty");

    let pubspec_path = base_path.join("pubspec.yaml");
    let content = fs::read_to_string(&pubspec_path)
        .with_context(|| format!("Failed to read {:?}", pubspec_path))?;

    if content.contains(&format!("{}:", package_name)) {
        return Ok(());
    }

    let git_dep = format!(
        "  {}:\n    git:\n      url: {}\n",
        package_name, git_url
    );

    let marker = "dependencies:\n";
    let updated = content
        .find(marker)
        .map(|pos| {
            let insert_at = pos + marker.len();
            let mut s = content.clone();
            s.insert_str(insert_at, &git_dep);
            s
        })
        .ok_or_else(|| anyhow::anyhow!("Could not find `dependencies:` in pubspec.yaml"))?;

    fs::write(&pubspec_path, updated)
        .with_context(|| format!("Failed to write {:?}", pubspec_path))?;

    Ok(())
}