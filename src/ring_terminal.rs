use csgrs::CSG;

/// Representation of a Ring Terminal module
#[derive(Debug, Clone)]
pub struct RingTerminal {
    pub name: String,
    pub outer_diameter: f64,
    pub inner_diameter: f64,
    pub terminal_length: f64,
    pub terminal_width: f64,
    pub terminal_thickness: f64,
}

impl RingTerminal {
    /// Generate the ring of the terminal
    pub fn ring(&self) -> CSG<()> {
        let outer = CSG::cylinder(self.outer_diameter / 2.0, self.terminal_thickness);
        let inner = CSG::cylinder(self.inner_diameter / 2.0, self.terminal_thickness);
        
        outer.subtract(&inner)
    }
    
    /// Generate the terminal tab
    pub fn tab(&self) -> CSG<()> {
        CSG::box_shape(self.terminal_length, self.terminal_width, self.terminal_thickness)
            .translate(0.0, self.outer_diameter / 2.0, 0.0)
    }
    
    /// Generate the full ring terminal model
    pub fn assemble(&self) -> CSG<()> {
        let mut terminal = self.ring();
        let tab = self.tab();
        
        terminal = terminal.union(&tab);
        
        terminal
    }
}

