use csgrs::CSG;

/// Representation of a Tubing module
#[derive(Debug, Clone)]
pub struct Tubing {
    pub name: String,
    pub outer_diameter: f64,
    pub inner_diameter: f64,
    pub length: f64,
}

impl Tubing {
    /// Generate the outer shell of the tubing
    pub fn outer_shell(&self) -> CSG<()> {
        CSG::cylinder(self.outer_diameter / 2.0, self.length)
    }
    
    /// Generate the inner bore of the tubing
    pub fn inner_bore(&self) -> CSG<()> {
        CSG::cylinder(self.inner_diameter / 2.0, self.length)
    }
    
    /// Generate the full tubing model
    pub fn assemble(&self) -> CSG<()> {
        let mut tubing = self.outer_shell();
        let bore = self.inner_bore();
        
        tubing = tubing.subtract(&bore);
        
        tubing
    }
}
