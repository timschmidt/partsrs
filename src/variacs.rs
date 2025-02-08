use csgrs::CSG;

/// Representation of a Variac Transformer
#[derive(Debug, Clone)]
pub struct Variac {
    pub base_diameter: f64,
    pub height: f64,
    pub knob_diameter: f64,
    pub knob_height: f64,
}

impl Variac {
    /// Generate the transformer base
    pub fn base(&self) -> CSG<()> {
        CSG::cylinder_z(self.base_diameter / 2.0, self.height)
    }

    /// Generate the adjustment knob
    pub fn knob(&self) -> CSG<()> {
        CSG::cylinder_z(self.knob_diameter / 2.0, self.knob_height)
            .translate(Vector3::new(0.0, 0.0, self.height))
    }

    /// Assemble the full Variac transformer
    pub fn assemble(&self) -> CSG<()> {
        self.base().union(&self.knob())
    }
}

