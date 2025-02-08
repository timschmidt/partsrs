use csgrs::CSG;

/// Representation of a Ribbon Cable Clamp
#[derive(Debug, Clone)]
pub struct RibbonClamp {
    pub width: f64,
    pub height: f64,
    pub depth: f64,
    pub slot_width: f64,
    pub slot_height: f64,
}

impl RibbonClamp {
    /// Generate the clamp body
    pub fn body(&self) -> CSG<()> {
        CSG::prism(self.width, self.height, self.depth)
    }

    /// Generate the ribbon slot
    pub fn slot(&self) -> CSG<()> {
        CSG::prism(self.slot_width, self.height, self.slot_height)
            .translate(0.0, 0.0, self.depth / 2.0)
    }

    /// Assemble the complete ribbon clamp
    pub fn assemble(&self) -> CSG<()> {
        self.body().difference(&self.slot())
    }
}

