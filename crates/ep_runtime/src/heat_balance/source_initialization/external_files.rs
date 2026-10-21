//! Reached Native CheckForActualFilePath and binary readFile operands.
//! This owner never eagerly reads a Schedule:File or canonicalizes its cache key.
use std::io::{self, Write};
use std::path::{Component, Path, PathBuf};

/// Same selected members of SystemVarsData, not a second process-variable owner.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct SourceFileSearchState {
    pub(crate) first_time: bool,
    pub(crate) env_input_path1: PathBuf,
    pub(crate) env_input_path2: PathBuf,
    pub(crate) test_all_paths: bool,
}
impl Default for SourceFileSearchState {
    fn default() -> Self {
        Self {
            first_time: true,
            env_input_path1: PathBuf::new(),
            env_input_path2: PathBuf::new(),
            test_all_paths: false,
        }
    }
}

/// std::filesystem::path component identity, preserving dot/parent/case spelling.
/// Rust Path::components removes interior dots, so PathBuf Ord is not this key.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct SourceNativePathKey(Vec<String>);
impl SourceNativePathKey {
    pub(crate) fn new(path: &Path) -> io::Result<Self> {
        let spelling = path.to_str().ok_or_else(|| {
            io::Error::new(io::ErrorKind::Unsupported, "non-Unicode Native path key")
        })?;
        let spelling = if cfg!(windows) {
            spelling.replace('\\', "/")
        } else {
            spelling.to_owned()
        };
        let mut rest = spelling.as_str();
        let mut pieces = Vec::new();
        if cfg!(windows) {
            if rest.as_bytes().get(1) == Some(&b':') {
                pieces.push(rest[..2].to_owned());
                rest = &rest[2..];
            } else if let Some(server) = rest.strip_prefix("//") {
                let end = server.find('/').unwrap_or(server.len());
                pieces.push(format!("//{}", &server[..end]));
                rest = &server[end..];
            }
        }
        if rest.starts_with('/') {
            pieces.push("/".into());
            rest = rest.trim_start_matches('/');
        }
        pieces.extend(rest.split('/').filter(|s| !s.is_empty()).map(str::to_owned));
        if !rest.is_empty() && rest.ends_with('/') {
            pieces.push(String::new());
        }
        Ok(Self(pieces))
    }
}

/// Genuine retained DataStringGlobals paths and IOFiles TempFullFilePath.
#[derive(Debug)]
pub(crate) struct SourceExternalInputFiles {
    input_dir_path: PathBuf,
    exe_directory_path: PathBuf,
    current_working_folder: PathBuf,
    program_path: PathBuf,
    pub(crate) temp_full_file_path: PathBuf,
}

pub(crate) struct SourceFileSearchResult {
    pub(crate) found: Option<PathBuf>,
    pub(crate) paths_searched: Vec<(&'static str, PathBuf)>,
}

fn native_path(path: &Path) -> io::Result<PathBuf> {
    let text = path
        .to_str()
        .ok_or_else(|| io::Error::new(io::ErrorKind::Unsupported, "non-Unicode makeNativePath"))?;
    Ok(PathBuf::from(if cfg!(windows) {
        text.replace('/', "\\")
    } else {
        text.replace('\\', "/")
    }))
}

fn parent_directory(path: &Path) -> io::Result<PathBuf> {
    let spelling = path.to_str().ok_or_else(|| {
        io::Error::new(io::ErrorKind::Unsupported, "non-Unicode Native parent path")
    })?;
    // Native strips *both* trailing separator spellings before parent_path().
    // Work on the spelling: Rust Path::parent removes interior `.` components.
    let stripped = spelling.trim_end_matches(['/', '\\']);
    let separator = |ch| ch == '/' || cfg!(windows) && ch == '\\';
    let root = if cfg!(windows) && stripped.as_bytes().get(1) == Some(&b':') {
        if stripped
            .as_bytes()
            .get(2)
            .is_some_and(|ch| *ch == b'/' || *ch == b'\\')
        {
            3
        } else {
            2
        }
    } else if cfg!(windows) && (stripped.starts_with("\\\\") || stripped.starts_with("//")) {
        stripped[2..]
            .find(separator)
            .map_or(stripped.len(), |at| at + 3)
    } else if stripped.starts_with(separator) {
        1
    } else {
        0
    };
    let parent = match stripped.rfind(separator) {
        Some(at) if at < root => &stripped[..root.min(stripped.len())],
        Some(at) => stripped[..at].trim_end_matches(separator),
        None if root != 0 => &stripped[..root.min(stripped.len())],
        None => "",
    };
    Ok(PathBuf::from(if parent.is_empty() { "./" } else { parent }))
}

/// Native FileSystem154-199: absolute display path is separate from cache identity.
fn absolute_path(path: &Path) -> io::Result<PathBuf> {
    let mut absolute = if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir()?.join(path)
    };
    if !absolute.is_absolute() {
        return Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "actual Native drive-relative absolute-path expansion",
        ));
    }
    while absolute.is_symlink() {
        let target = std::fs::read_link(&absolute)?;
        absolute = if target.is_absolute() {
            target
        } else {
            parent_directory(&absolute)?.join(target)
        };
    }
    let mut result = PathBuf::new();
    for part in absolute.components() {
        match part {
            Component::CurDir => {}
            Component::ParentDir => {
                if result.is_symlink() || result.file_name().is_some_and(|s| s == "..") {
                    result.push("..");
                } else {
                    result.pop();
                }
            }
            _ => result.push(part.as_os_str()),
        }
    }
    Ok(result)
}

pub(crate) fn display_native_path(path: &Path) -> io::Result<&str> {
    path.to_str().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::Unsupported,
            "non-Unicode Native path diagnostic",
        )
    })
}

impl SourceExternalInputFiles {
    /// Same CLI input parent, actual current process executable parent, constructor
    /// empty INI/path slots; no sidecar or environment lookup occurs at construction.
    pub(crate) fn new(source_input_file: &Path) -> io::Result<Self> {
        let executable = absolute_path(&std::env::current_exe()?)?;
        Ok(Self {
            input_dir_path: parent_directory(source_input_file)?,
            exe_directory_path: parent_directory(&executable)?,
            current_working_folder: PathBuf::new(),
            program_path: PathBuf::new(),
            temp_full_file_path: PathBuf::new(),
        })
    }

    /// Native DataSystemVariables117-205. The already-open real AUD is borrowed
    /// independently; every reached failed/found probe is written immediately.
    pub(crate) fn resolve(
        &mut self,
        original: &Path,
        state: &mut SourceFileSearchState,
        audit: &mut dyn Write,
    ) -> io::Result<SourceFileSearchResult> {
        if state.first_time {
            state.env_input_path1 =
                parent_directory(&PathBuf::from(std::env::var_os("epin").unwrap_or_default()))?;
            state.env_input_path2 =
                PathBuf::from(std::env::var_os("input_path").unwrap_or_default());
            self.program_path = PathBuf::from(std::env::var_os("program_path").unwrap_or_default());
            state.first_time = false;
        }
        let input = native_path(original)?;
        let candidates = [
            (input.clone(), "Current Working Directory"),
            (self.input_dir_path.join(&input), "IDF Directory"),
            (
                self.exe_directory_path.join(&input),
                "EnergyPlus Executable Directory",
            ),
            (
                state.env_input_path1.join(&input),
                "\"epin\" Environment Variable",
            ),
            (
                state.env_input_path2.join(&input),
                "\"input_path\" Environment Variable",
            ),
            (
                self.current_working_folder.join(&input),
                "INI File Directory",
            ),
            (
                self.program_path.join(&input),
                "\"program\", \"dir\" from INI File",
            ),
        ];
        let count = if state.test_all_paths { 7 } else { 5 };
        let mut paths_searched: Vec<(&'static str, PathBuf)> = Vec::new();
        for (candidate, label) in &candidates[..count] {
            // Native fileExists is exists && !is_directory (not is_regular_file).
            let exists = candidate.try_exists()? && !std::fs::metadata(candidate)?.is_dir();
            let absolute = absolute_path(candidate)?;
            if exists {
                writeln!(audit, "found ({label})={}", display_native_path(&absolute)?)?;
                self.temp_full_file_path = candidate.clone();
                return Ok(SourceFileSearchResult {
                    found: Some(candidate.clone()),
                    paths_searched,
                });
            }
            let parent = parent_directory(&absolute)?;
            let parent_key = SourceNativePathKey::new(&parent)?;
            if !paths_searched
                .iter()
                .any(|(_, p)| SourceNativePathKey::new(p).is_ok_and(|key| key == parent_key))
            {
                paths_searched.push((*label, parent));
            }
            writeln!(
                audit,
                "not found ({label})={}",
                display_native_path(&absolute)?
            )?;
        }
        Ok(SourceFileSearchResult {
            found: None,
            paths_searched,
        })
    }

    /// Native FileSystem347-366 reads the whole physical file in binary mode.
    pub(crate) fn read_bytes(&self) -> io::Result<Vec<u8>> {
        std::fs::read(&self.temp_full_file_path)
    }
}
