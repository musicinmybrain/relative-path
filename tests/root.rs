use std::fs;
use std::path::{Path, PathBuf};

use anyhow::Result;
use relative_path::Root;

const PATH: &str = env!("CARGO_TARGET_TMPDIR");

fn make_path(path: &'static str) -> PathBuf {
    Path::new(PATH).join(path)
}

fn root(path: &'static str) -> Root {
    match Root::new(make_path(path)) {
        Ok(root) => root,
        Err(error) => panic!("Failed to open root: {}: {}", path, error),
    }
}

fn files(list: &[(&'static str, Option<&'static str>)]) {
    for (item, content) in list {
        let path = make_path(item);

        if let Some(content) = content {
            if let Some(parent) = path.parent() {
                if let Err(error) = fs::create_dir_all(parent) {
                    panic!("Failed to create directory {}: {}", parent.display(), error);
                }
            }

            if let Err(error) = fs::write(&path, content) {
                panic!("Failed to create file {}: {}", path.display(), error);
            }
        } else if let Err(error) = fs::create_dir_all(&path) {
            panic!("Failed to create directory {}: {}", path.display(), error);
        }
    }
}

#[test]
fn relative_open() -> Result<()> {
    files(&[("relative_open/src/root/first", Some("first content"))]);

    let r1 = root("src/root");
    assert_eq!(r1.read_to_string("first")?, "first content");

    let r2 = root("src");
    assert_eq!(r2.read_to_string("root/first")?, "first content");
    Ok(())
}

#[test]
fn read_dir() -> Result<()> {
    files(&[("read_dir/src/root/first", Some("first content"))]);
    files(&[("read_dir/src/root/second", Some("second content"))]);

    let r1 = root(".");
    let d = r1.read_dir("read_dir/src/root")?;

    let mut values = Vec::new();

    for e in d {
        let e = e?;
        values.push(e.file_name().to_string_lossy().into_owned());
    }

    values.sort();

    assert_eq!(values, vec!["first", "second"]);
    Ok(())
}

#[test]
fn glob() -> Result<()> {
    files(&[("glob/src/root/first", Some("first content"))]);
    files(&[("glob/src/root/second", Some("second content"))]);

    let r1 = root("glob");
    let glob = r1.glob("**/root/*")?;

    let mut results = Vec::new();

    for e in glob.matcher() {
        results.push(e?);
    }

    results.sort();
    assert_eq!(results, vec!["src/root/first", "src/root/second"]);
    Ok(())
}