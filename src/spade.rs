use csgrs::CSG;

/// Representation of a Spade Terminal module
#[derive(Debug, Clone)]
pub struct SpadeTerminal {
    pub name: String,
    pub blade_width: f64,
    pub blade_thickness: f64,
    pub blade_length: f64,
    pub barrel_diameter: f64,
    pub barrel_length: f64,
}

impl SpadeTerminal {
    /// Generate the blade of the spade terminal
    pub fn blade(&self) -> CSG<()> {
        CSG::prism(self.blade_width, self.blade_thickness, self.blade_length)
    }
    
    /// Generate the barrel of the spade terminal
    pub fn barrel(&self) -> CSG<()> {
        CSG::cylinder(self.barrel_diameter / 2.0, self.barrel_length)
            .translate(0.0, 0.0, self.blade_length)
    }
    
    /// Generate the full spade terminal model
    pub fn assemble(&self) -> CSG<()> {
        let mut terminal = self.blade();
        let barrel = self.barrel();
        
        terminal = terminal.union(&barrel);
        
        terminal
    }
}

