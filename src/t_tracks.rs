use csgrs::CSG;

/// Representation of a T-Track
#[derive(Debug, Clone)]
pub struct TTrack {
    pub length: f64,
    pub base_width: f64,
    pub base_height: f64,
    pub slot_width: f64,
    pub slot_depth: f64,
}

impl TTrack {
    /// Generate the base of the T-track
    pub fn base(&self) -> CSG<()> {
        CSG::box_shape(self.base_width, self.length, self.base_height)
    }

    /// Generate the T-slot
    pub fn slot(&self) -> CSG<()> {
        CSG::box_shape(self.slot_width, self.length, self.slot_depth)
            .translate(0.0, 0.0, self.base_height / 2.0)
    }

    /// Assemble the complete T-track
    pub fn assemble(&self) -> CSG<()> {
        self.base().difference(&self.slot())
    }
}

