use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::{fs, io};

use indoc::formatdoc;
use once_cell::sync::OnceCell;

use crate::env;

fn setup() -> color_eyre::Result<PathBuf> {
    static SETUP: OnceCell<PathBuf> = OnceCell::new();
    let path = SETUP.get_or_try_init(|| {
        let path = env::RTX_DATA_DIR.join(".fake-asdf");
        let asdf_bin = path.join("asdf");
        if !asdf_bin.exists() {
            fs::create_dir_all(&path)?;
            fs::write(
                &asdf_bin,
                formatdoc! {r#"
                #!/bin/sh
                rtx="${{RTX_EXE:-rtx}}"
                "$rtx" asdf "$@"
            "#},
            )?;
            let mut perms = asdf_bin.metadata()?.permissions();
            perms.set_mode(0o755);
            fs::set_permissions(&asdf_bin, perms)?;
        }
        Ok::<PathBuf, io::Error>(path)
    })?;

    Ok(path.clone())
}

pub fn get_path_with_fake_asdf() -> String {
    let mut path = vec![];
    match setup() {
        Ok(fake_asdf_path) => {
            path.push(fake_asdf_path.to_string_lossy().to_string());
        }
        Err(e) => {
            warn!("Failed to setup fake asdf: {}", e);
        }
    };
    path.push(env::PATH.to_string_lossy().to_string());
    path.join(":")
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_setup() {
        let path = setup().unwrap();
        assert!(path.join("asdf").exists());
        fs::remove_dir_all(&path).unwrap();
    }
}
