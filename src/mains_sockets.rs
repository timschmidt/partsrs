use csgrs::CSG;

/// Representation of a Mains Socket
#[derive(Debug, Clone)]
pub struct MainsSocket {
    pub width: f64,
    pub height: f64,
    pub depth: f64,
    pub hole_diameter: f64,
    pub hole_spacing: f64,
}

impl MainsSocket {
    /// Generate the socket body
    pub fn body(&self) -> CSG<()> {
        CSG::prism(self.width, self.height, self.depth)
    }

    /// Generate the plug holes
    pub fn plug_holes(&self) -> CSG<()> {
        let hole = CSG::cylinder(self.hole_diameter / 2.0, self.depth + 1.0);
        hole.translate(-self.hole_spacing / 2.0, 0.0, 0.0)
            .union(&hole.translate(self.hole_spacing / 2.0, 0.0, 0.0))
    }

    /// Assemble the complete mains socket
    pub fn assemble(&self) -> CSG<()> {
        self.body().difference(&self.plug_holes())
    }
}

