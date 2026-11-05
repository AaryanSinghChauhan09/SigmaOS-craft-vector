#[derive(Debug, Clone)]
pub struct VectorNode { pub x: f64, pub y: f64 }

#[derive(Debug, Default)]
pub struct VectorCurve { nodes: Vec<VectorNode> }
impl VectorCurve {
    pub fn new() -> Self { Self::default() }
    pub fn push(&mut self, x: f64, y: f64) { self.nodes.push(VectorNode { x, y }); }
    pub fn count(&self) -> usize { self.nodes.len() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_curve() {
        let mut c = VectorCurve::new();
        c.push(10.0, 20.0);
        assert_eq!(c.count(), 1);
    }
}
