use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;

use assert_cmd::assert::Assert;
use assert_cmd::Command;
use predicates::prelude::*;
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

#[test]
fn test_stowsave_directory() {
    Test::new()
        .with_file(".config/nvim/init.vim", "contents")
        .with_dir("stow/package")
        .run(["stowsave", ".config", "stow/package"])
        .assert_file(".config.bak/nvim/init.vim", "contents")
        .assert_file("stow/package/.config/nvim/init.vim", "contents")
        .assert_symlink(".config", "stow/package/.config");
}

#[test]
fn test_stowsave_nonexistent_path() {
    Test::new()
        .with_dir("stow/package")
        .run(["stowsave", "nonexistent", "stow/package"])
        .assert_failed()
        .assert_stderr_contains("Path does not exist")
        .assert_dir("stow/package")
        .assert_not_path("nonexistent");
}

#[test]
fn test_stowsave_invalid_stow_directory() {
    Test::new()
        .with_file(".vimrc", "contents")
        .run(["stowsave", ".vimrc", "invalid_stow_dir"])
        .cmd_result
        .failure()
        .stderr(predicate::str::contains("Stow package does not exist"));
}

#[test]
fn test_stowsave_symlink() {
    Test::new()
        .with_file(".vimrc", "contents")
        .with_symlink(".vimrc_link", ".vimrc")
        .with_dir("stow/package")
        .run(["stowsave", ".vimrc_link", "stow/package"])
        .assert_failed()
        .assert_stderr_contains("Cannot save symlinks");
}

struct Output {
    cmd_result: Assert,
    temp_dir: TempDir,
}

#[derive(Default)]
struct Test {
    dirs: Vec<PathBuf>,
    files: Vec<(PathBuf, String)>,     // (file, contents)
    symlinks: Vec<(PathBuf, PathBuf)>, // (symlink, target)
}

impl Test {
    fn new() -> Self {
        Self::default()
    }
    fn with_dir(mut self, dir: impl Into<PathBuf>) -> Self {
        self.dirs.push(dir.into());
        self
    }
    fn with_file(mut self, file: impl Into<PathBuf>, contents: impl Into<String>) -> Self {
        self.files.push((file.into(), contents.into()));
        self
    }
    fn with_symlink(mut self, file: impl Into<PathBuf>, target: impl Into<PathBuf>) -> Self {
        self.symlinks.push((file.into(), target.into()));
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
        // create symlinks:
        for s in self.symlinks {
            let symlink_path = temp_dir.path().join(s.0);
            let parent_dir = symlink_path.parent().unwrap();
            let target = temp_dir.path().join(s.1);
            // - create parent directory
            fs::create_dir_all(parent_dir).unwrap();
            // - create symlink
            std::os::unix::fs::symlink(&target, &symlink_path).unwrap();
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
        let cmd_result = cmd.assert(); // this actually runs the command

        Output {
            temp_dir,
            cmd_result,
        }
    }
}

// we'll want to move these methods to `impl Test` at some point
impl Output {
    fn assert_file(self, file: impl Into<PathBuf>, contents: impl Into<String>) -> Self {
        let file_path = self.temp_dir.path().join(file.into());
        assert!(file_path.exists());
        let file_contents = fs::read_to_string(&file_path).unwrap();
        assert!(file_contents == contents.into());
        self
    }
    fn assert_dir(self, dir: impl Into<PathBuf>) -> Self {
        let dir_path = self.temp_dir.path().join(dir.into());
        assert!(dir_path.is_dir());
        self
    }
    fn assert_not_path(self, path: impl Into<PathBuf>) -> Self {
        let path = self.temp_dir.path().join(path.into());
        assert!(!path.exists());
        self
    }
    fn assert_symlink(self, file: impl Into<PathBuf>, target: impl Into<PathBuf>) -> Self {
        let file_path = self.temp_dir.path().join(file.into());
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
    fn assert_failed(mut self) -> Self {
        self.cmd_result = self.cmd_result.failure();
        self
    }
    fn assert_stderr_contains(mut self, expected: impl Into<String>) -> Self {
        self.cmd_result = self
            .cmd_result
            .stderr(predicate::str::contains(expected.into()));
        self
    }
}
