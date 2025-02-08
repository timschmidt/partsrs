use csgrs::CSG;

/// Representation of a Box Section module
#[derive(Debug, Clone)]
pub struct BoxSection {
    pub name: String,
    pub outer_width: f64,
    pub outer_height: f64,
    pub outer_length: f64,
    pub wall_thickness: f64,
}

impl BoxSection {
    /// Generate the outer shell of the box section
    pub fn outer_shell(&self) -> CSG<()> {
        CSG::prism(self.outer_width, self.outer_height, self.outer_length)
    }
    
    /// Generate the inner hollow section
    pub fn inner_hollow(&self) -> CSG<()> {
        CSG::prism(
            self.outer_width - 2.0 * self.wall_thickness,
            self.outer_height - 2.0 * self.wall_thickness,
            self.outer_length,
        )
    }
    
    /// Generate the full box section model
    pub fn assemble(&self) -> CSG<()> {
        let mut box_section = self.outer_shell();
        let hollow = self.inner_hollow();
        
        box_section = box_section.subtract(&hollow);
        
        box_section
    }
}

