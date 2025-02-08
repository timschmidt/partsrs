use csgrs::CSG;

/// Representation of a Sealing Strip
#[derive(Debug, Clone)]
pub struct SealingStrip {
    pub length: f64,
    pub width: f64,
    pub height: f64,
    pub groove_width: f64,
    pub groove_depth: f64,
}

impl SealingStrip {
    /// Generate the strip body
    pub fn body(&self) -> CSG<()> {
        CSG::prism(self.width, self.length, self.height)
    }

    /// Generate the groove in the strip
    pub fn groove(&self) -> CSG<()> {
        CSG::prism(self.groove_width, self.length, self.groove_depth)
            .translate(0.0, 0.0, -self.groove_depth / 2.0)
    }

    /// Assemble the complete sealing strip
    pub fn assemble(&self) -> CSG<()> {
        self.body().difference(&self.groove())
    }
}

