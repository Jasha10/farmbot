use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;

use assert_cmd::Command;
// use predicates::prelude::*;
use tempfile::TempDir;

#[test]
fn test_stowsave_single_file() {
    Test::new()
        .with_file(".vimrc", "contents")
        .with_dir("stow/package")
        .run(["stowsave", ".vimrc", "stow/package"])
        .assert_file(".vimrc.bak", "contents")
        .assert_file("stow/package/.vimrc", "contents")
        .assert_symlink(".vimrc", "stow/package/.vimrc");
}

struct Output {
    temp_dir: TempDir,
}

#[derive(Default)]
struct Test {
    dirs: Vec<String>,
    files: Vec<(String, String)>, // (file, contents)
}

impl Test {
    fn new() -> Self {
        Self::default()
    }
    fn with_dir(mut self, dir: impl Into<String>) -> Self {
        self.dirs.push(dir.into());
        self
    }
    fn with_file(mut self, file: impl Into<String>, contents: impl Into<String>) -> Self {
        self.files.push((file.into(), contents.into()));
        self
    }
    fn run(self, args: impl IntoIterator<Item = impl Into<String>>) -> Output {
        //// SETUP
        // create temporary directory
        let temp_dir = TempDir::new().unwrap();
        // create dirs within temporary directory
        for dir in self.dirs {
            fs::create_dir_all(temp_dir.path().join(dir)).unwrap();
        }
        // create files:
        for f in self.files {
            let file_path = temp_dir.path().join(f.0);
            let parent_dir = file_path.parent().unwrap();
            // - create parent directory
            fs::create_dir_all(parent_dir).unwrap();
            // - create file
            let mut file = File::create(&file_path).unwrap();
            write!(file, "{}", f.1).unwrap();
        }

        //// RUN
        // collect args
        let args: Vec<String> = args.into_iter().map(Into::into).collect();
        if args.is_empty() {
            panic!("No args provided");
        }
        let mut cmd = Command::cargo_bin(&args[0]).unwrap();
        for arg in args.iter().skip(1) {
            cmd.arg(arg);
        }
        cmd.current_dir(temp_dir.path());
        cmd.assert();

        Output { temp_dir }
    }
}

// we'll want to move these methods to `impl Test` at some point
impl Output {
    fn assert_file(self, file: impl Into<String>, contents: impl Into<String>) -> Self {
        let file = file.into();
        let file_path = self.temp_dir.path().join(file);
        assert!(file_path.exists());
        let file_contents = fs::read_to_string(&file_path).unwrap();
        assert!(file_contents == contents.into());
        self
    }
    fn assert_symlink(self, file: impl Into<String>, target: impl Into<PathBuf>) -> Self {
        let file = file.into();
        let file_path = self.temp_dir.path().join(file);
        assert!(file_path.is_symlink());
        let symlink_target = fs::read_link(&file_path).unwrap();
        let target = target.into();
        assert!(
            symlink_target == target,
            "Expected {:?}, got {:?}",
            target,
            symlink_target
        );
        self
    }
}

// #[test]
// fn test_stowsave_directory() -> Result<(), Box<dyn std::error::Error>> {
//     let temp_dir = TempDir::new()?;
//     let home_dir = temp_dir.path().join("home");
//     let stow_dir = temp_dir.path().join("stow");
//
//     fs::create_dir_all(&home_dir)?;
//     fs::create_dir_all(&stow_dir)?;
//
//     let config_dir = home_dir.join(".config");
//     fs::create_dir_all(&config_dir)?;
//
//     let nvim_dir = config_dir.join("nvim");
//     fs::create_dir_all(&nvim_dir)?;
//
//     let init_vim_path = nvim_dir.join("init.vim");
//     let mut init_vim_file = File::create(&init_vim_path)?;
//     writeln!(init_vim_file, "set relativenumber")?;
//
//     let mut cmd = Command::cargo_bin("stowsave")?;
//     cmd.arg(&config_dir).arg(&stow_dir);
//     cmd.assert().success();
//
//     assert!(config_dir.with_extension("bak").exists());
//     assert!(stow_dir
//         .join(".config")
//         .join("nvim")
//         .join("init.vim")
//         .exists());
//     assert!(config_dir.is_symlink());
//
//     Ok(())
// }
//
// #[test]
// fn test_stowsave_nonexistent_path() -> Result<(), Box<dyn std::error::Error>> {
//     let temp_dir = TempDir::new()?;
//     let home_dir = temp_dir.path().join("home");
//     let stow_dir = temp_dir.path().join("stow");
//
//     fs::create_dir_all(&home_dir)?;
//     fs::create_dir_all(&stow_dir)?;
//
//     let nonexistent_path = home_dir.join("nonexistent");
//
//     let mut cmd = Command::cargo_bin("stowsave")?;
//     cmd.arg(&nonexistent_path).arg(&stow_dir);
//     cmd.assert()
//         .failure()
//         .stderr(predicate::str::contains("Path does not exist"));
//
//     Ok(())
// }
//
// #[test]
// fn test_stowsave_invalid_stow_directory() -> Result<(), Box<dyn std::error::Error>> {
//     let temp_dir = TempDir::new()?;
//     let home_dir = temp_dir.path().join("home");
//     let invalid_stow_dir = temp_dir.path().join("invalid_stow");
//
//     fs::create_dir_all(&home_dir)?;
//
//     let vimrc_path = home_dir.join(".vimrc");
//     let mut vimrc_file = File::create(&vimrc_path)?;
//     writeln!(vimrc_file, "set number")?;
//
//     let mut cmd = Command::cargo_bin("stowsave")?;
//     cmd.arg(&vimrc_path).arg(&invalid_stow_dir);
//     cmd.assert()
//         .failure()
//         .stderr(predicate::str::contains("Invalid stow directory"));
//
//     Ok(())
// }
//
// #[test]
// fn test_stowsave_symlink() -> Result<(), Box<dyn std::error::Error>> {
//     let temp_dir = TempDir::new()?;
//     let home_dir = temp_dir.path().join("home");
//     let stow_dir = temp_dir.path().join("stow");
//
//     fs::create_dir_all(&home_dir)?;
//     fs::create_dir_all(&stow_dir)?;
//
//     let vimrc_path = home_dir.join(".vimrc");
//     let mut vimrc_file = File::create(&vimrc_path)?;
//     writeln!(vimrc_file, "set number")?;
//
//     let symlink_path = home_dir.join(".vimrc_link");
//     std::os::unix::fs::symlink(&vimrc_path, &symlink_path)?;
//
//     let mut cmd = Command::cargo_bin("stowsave")?;
//     cmd.arg(&symlink_path).arg(&stow_dir);
//     cmd.assert()
//         .failure()
//         .stderr(predicate::str::contains("Cannot save symlinks"));
//
//     Ok(())
// }
