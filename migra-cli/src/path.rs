use std::ffi::OsStr;
use std::path::{Path, PathBuf};

#[derive(Debug, Default)]
pub struct PathBuilder {
    buf: PathBuf,
}

impl<P: AsRef<Path>> From<P> for PathBuilder {
    fn from(path: P) -> Self {
        PathBuilder {
            buf: path.as_ref().to_path_buf(),
        }
    }
}

impl PathBuilder {
    pub fn append<P: AsRef<Path>>(&mut self, path: P) -> &mut Self {
        self.buf.push(path);
        self
    }

    pub fn default_extension<S: AsRef<OsStr>>(&mut self, extension: S) -> &mut Self {
        if self.buf.as_path().extension().is_none() {
            self.buf.set_extension(extension);
        }
        self
    }

    pub fn build(&self) -> PathBuf {
        self.buf.clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::path::Path;
    use crate::path::PathBuilder;

    #[test]
    fn create_path_builder() {
        let path = PathBuilder::from("test").build();

        assert_eq!(path, Path::new("test"))
    }

    #[test]
    fn append_path_to_builder() {
        let path = PathBuilder::from("directory").append("schema.sql").build();

        assert_eq!(path, Path::new("directory/schema.sql"))
    }

    #[test]
    fn add_default_extension_for_path() {
        let path = PathBuilder::from("directory")
            .append("schema")
            .default_extension("sql")
            .build();

        assert_eq!(path, Path::new("directory/schema.sql"));
    }

    #[test]
    fn build_default_path() {
        let path = PathBuilder::default().build();

        assert_eq!(path, Path::new(""));
    }
}
