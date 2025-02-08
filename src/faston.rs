use csgrs::CSG;

/// Representation of a Faston Terminal
#[derive(Debug, Clone)]
pub struct FastonTerminal {
    pub width: f64,
    pub thickness: f64,
    pub length: f64,
    pub barrel_diameter: f64,
    pub barrel_length: f64,
}

impl FastonTerminal {
    /// Generate the flat blade of the terminal
    pub fn blade(&self) -> CSG<()> {
        CSG::box_shape(self.width, self.thickness, self.length)
    }

    /// Generate the crimp barrel
    pub fn barrel(&self) -> CSG<()> {
        CSG::cylinder(self.barrel_diameter / 2.0, self.barrel_length)
            .translate(0.0, 0.0, self.length)
    }

    /// Assemble the full Faston terminal
    pub fn assemble(&self) -> CSG<()> {
        self.blade().union(&self.barrel())
    }
}

