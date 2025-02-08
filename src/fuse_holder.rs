use csgrs::CSG;

/// Representation of a Fuse Holder
#[derive(Debug, Clone)]
pub struct FuseHolder {
    pub body_diameter: f64,
    pub body_length: f64,
    pub cap_diameter: f64,
    pub cap_length: f64,
    pub fuse_diameter: f64,
    pub fuse_length: f64,
}

impl FuseHolder {
    /// Generate the main body of the fuse holder
    pub fn body(&self) -> CSG<()> {
        CSG::cylinder(self.body_diameter / 2.0, self.body_length)
    }

    /// Generate the cap of the fuse holder
    pub fn cap(&self) -> CSG<()> {
        CSG::cylinder(self.cap_diameter / 2.0, self.cap_length)
            .translate(Vector3::new(0.0, 0.0, self.body_length))
    }

    /// Generate the fuse slot
    pub fn fuse_slot(&self) -> CSG<()> {
        CSG::cylinder(self.fuse_diameter / 2.0, self.fuse_length)
            .translate(Vector3::new(0.0, 0.0, self.body_length / 2.0))
    }

    /// Assemble the complete fuse holder
    pub fn assemble(&self) -> CSG<()> {
        self.body().union(&self.cap()).difference(&self.fuse_slot())
    }
}

