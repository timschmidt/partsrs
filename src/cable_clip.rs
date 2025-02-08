use csgrs::CSG;

/// Representation of a Cable Clip
#[derive(Debug, Clone)]
pub struct CableClip {
    pub width: f64,
    pub height: f64,
    pub depth: f64,
    pub cable_diameter: f64,
    pub hole_diameter: f64,
}

impl CableClip {
    /// Generate the clip body
    pub fn body(&self) -> CSG<()> {
        CSG::prism(self.width, self.height, self.depth)
    }

    /// Generate the cable slot
    pub fn cable_slot(&self) -> CSG<()> {
        CSG::cylinder(self.cable_diameter / 2.0, self.depth)
            .translate(Vector3::new(0.0, self.height / 4.0, 0.0))
    }

    /// Generate the mounting hole
    pub fn hole(&self) -> CSG<()> {
        CSG::cylinder(self.hole_diameter / 2.0, self.depth + 1.0)
            .translate(Vector3::new(0.0, -self.height / 4.0, 0.0))
    }

    /// Assemble the complete cable clip
    pub fn assemble(&self) -> CSG<()> {
        self.body().difference(&self.cable_slot()).difference(&self.hole())
    }
}

