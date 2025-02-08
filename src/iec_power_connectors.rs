use csgrs::CSG;

/// Representation of an IEC Power Connector
#[derive(Debug, Clone)]
pub struct IECConnector {
    pub name: String,
    pub width: f64,
    pub height: f64,
    pub depth: f64,
    pub mount_hole_spacing: f64,
}

impl IECConnector {
    /// Generate the main body of the IEC connector
    pub fn body(&self) -> CSG<()> {
        CSG::prism(self.width, self.height, self.depth)
    }

    /// Generate the mounting holes
    pub fn mount_holes(&self) -> CSG<()> {
        let hole = CSG::cylinder(3.0, self.depth); // Assuming M3 screws
        hole.translate(self.mount_hole_spacing / 2.0, 0.0, 0.0)
            .union(&hole.translate(-self.mount_hole_spacing / 2.0, 0.0, 0.0))
    }

    /// Assemble the final IEC connector
    pub fn assemble(&self) -> CSG<()> {
        self.body().difference(&self.mount_holes())
    }
}

