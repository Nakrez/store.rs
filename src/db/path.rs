use db::data::Data;

/// A part of a Path
pub enum PathPart {
    Binary(Data),
}

/// Represents a path in the database
pub struct Path {
    parts: Vec<PathPart>,
}

impl Path {
    /// Create a new path from parts
    pub fn new(parts: Vec<PathPart>) -> Self {
        Path {
            parts: parts,
        }
    }

    /// Get the parts that compose the path
    pub fn parts(&self) -> &Vec<PathPart> {
        &self.parts
    }
}

/// Trait for converting a type to a Path, consuming it in the process.
pub trait IntoPath {
    /// Converts a type to a Path
    fn into_path(self) -> Path;
}

impl IntoPath for String {
    fn into_path(self) -> Path {
        let mut parts = Vec::new();

        // XXX: Will need some work here, it doesn't support any feature
        // XXX: Escape ? Regex ? Globbing ?
        for p in self.split('/') {
            if p.is_empty() {
                continue;
            }

            let mut data_part = Vec::new();

            data_part.extend(p.as_bytes());

            let part = PathPart::Binary(data_part);

            parts.push(part);
        }

        Path::new(parts)
    }
}

impl<'a> IntoPath for &'a str {
    fn into_path(self) -> Path {
        self.to_string().into_path()
    }
}
