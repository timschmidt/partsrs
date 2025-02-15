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
        CSG::prism(self.width, self.thickness, self.length)
    }

    /// Generate the crimp barrel
    pub fn barrel(&self) -> CSG<()> {
        CSG::cylinder_z(self.barrel_diameter / 2.0, self.barrel_length)
            .translate(Vector3::new(0.0, 0.0, self.length))
    }

    /// Assemble the full Faston terminal
    pub fn assemble(&self) -> CSG<()> {
        self.blade().union(&self.barrel())
    }
}

