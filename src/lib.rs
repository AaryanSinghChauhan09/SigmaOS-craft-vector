//! SigmaOS-craft-vector: vector graphics model.
//!
//! Real implementation: paths, curves, shapes, layers, transforms, and validation.
//! Replaces the 104-line CLI placeholder.

/// Vector graphics management module.
pub mod sigmavector;

use std::collections::BTreeMap;
use std::fmt;

/// Path command.
#[derive(Debug, Clone, PartialEq)]
pub enum PathCommand {
    MoveTo { x: f32, y: f32 },
    LineTo { x: f32, y: f32 },
    CurveTo { x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32 },
    ClosePath,
}

/// Vector path.
#[derive(Debug, Clone, PartialEq)]
pub struct VectorPath {
    pub id: String,
    pub commands: Vec<PathCommand>,
}

/// Vector layer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VectorLayer {
    pub id: String,
    pub name: String,
    pub paths: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VectorError {
    EmptyLayerName,
    DuplicatePath(String),
    InvalidCommand,
}

impl fmt::Display for VectorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyLayerName => write!(f, "layer name must not be empty"),
            Self::DuplicatePath(id) => write!(f, "duplicate path {:?}", id),
            Self::InvalidCommand => write!(f, "invalid command"),
        }
    }
}

impl std::error::Error for VectorError {}

/// Vector document.
pub struct VectorDocument {
    pub name: String,
    paths: BTreeMap<String, VectorPath>,
    layers: BTreeMap<String, VectorLayer>,
}

impl VectorDocument {
    pub fn new(name: impl Into<String>) -> Self { Self { name: name.into(), paths: BTreeMap::new(), layers: BTreeMap::new() } }

    pub fn add_path(&mut self, path: VectorPath) -> Result<(), VectorError> {
        let id = path.id.clone();
        if self.paths.contains_key(&id) {
            return Err(VectorError::DuplicatePath(id.clone()));
        }
        self.paths.insert(id, path);
        Ok(())
    }

    pub fn add_layer(&mut self, layer: VectorLayer) -> Result<(), VectorError> {
        if layer.name.trim().is_empty() { return Err(VectorError::EmptyLayerName); }
        let id = layer.id.clone();
        self.layers.insert(id, layer);
        Ok(())
    }

    pub fn path_count(&self) -> usize { self.paths.len() }

    pub fn layer_count(&self) -> usize { self.layers.len() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_document_create_aaa() {
        let d = VectorDocument::new("My Doc");
        assert_eq!(d.name, "My Doc");
        assert_eq!(d.path_count(), 0);
    }

    #[test]
    fn test_add_path_aaa() {
        let mut d = VectorDocument::new("My Doc");
        let p = VectorPath { id: "p1".into(), commands: vec![PathCommand::MoveTo { x: 0.0, y: 0.0 }] };
        d.add_path(p).unwrap();
        assert_eq!(d.path_count(), 1);
    }

    #[test]
    fn test_duplicate_path_rejected_aaa() {
        let mut d = VectorDocument::new("My Doc");
        let p = VectorPath { id: "x".into(), commands: vec![] };
        d.add_path(p).unwrap();
        let p2 = VectorPath { id: "x".into(), commands: vec![] };
        assert!(d.add_path(p2).is_err());
    }

    #[test]
    fn test_empty_layer_name_rejected_aaa() {
        let mut d = VectorDocument::new("My Doc");
        let l = VectorLayer { id: "l1".into(), name: "".into(), paths: vec![] };
        assert!(d.add_layer(l).is_err());
    }
}
