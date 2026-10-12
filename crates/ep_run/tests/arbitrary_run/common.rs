use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use serde_json::Value;

static NEXT_CASE: AtomicU64 = AtomicU64::new(0);

pub(super) struct CaseDir(PathBuf);

impl Deref for CaseDir {
    type Target = Path;

    fn deref(&self) -> &Path {
        &self.0
    }
}

impl AsRef<Path> for CaseDir {
    fn as_ref(&self) -> &Path {
        &self.0
    }
}

impl Drop for CaseDir {
    fn drop(&mut self) {
        // Keep failed assertion artifacts available for diagnosis.
        if std::thread::panicking() {
            return;
        }
        let result = std::fs::remove_dir_all(&self.0);
        assert!(
            result.is_ok()
                || result
                    .as_ref()
                    .is_err_and(|error| error.kind() == std::io::ErrorKind::NotFound),
            "failed to clean test directory {}: {result:?}",
            self.0.display()
        );
    }
}

pub(super) fn unique_case_dir(name: &str) -> Result<CaseDir, Box<dyn std::error::Error>> {
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
    let sequence = NEXT_CASE.fetch_add(1, Ordering::Relaxed);
    let path = std::env::temp_dir().join(format!(
        "rusted-energyplus-ep-run-{name}-{}-{timestamp}-{sequence}",
        std::process::id()
    ));
    // Only a directory created by this call is owned and removed by the guard.
    std::fs::create_dir(&path)?;
    Ok(CaseDir(path))
}

pub(super) fn write_text(path: &Path, contents: &str) -> Result<(), Box<dyn std::error::Error>> {
    std::fs::write(path, contents)?;
    Ok(())
}

pub(super) fn read_json(path: &Path) -> Result<Value, Box<dyn std::error::Error>> {
    let contents = std::fs::read_to_string(path)?;
    Ok(serde_json::from_str(&contents)?)
}
