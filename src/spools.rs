use csgrs::CSG;

/// Representation of a Spool module
#[derive(Debug, Clone)]
pub struct Spool {
    pub name: String,
    pub outer_diameter: f64,
    pub inner_diameter: f64,
    pub width: f64,
    pub hub_diameter: f64,
    pub hub_width: f64,
}

impl Spool {
    /// Generate the outer shell of the spool
    pub fn outer_shell(&self) -> CSG<()> {
        CSG::cylinder(self.outer_diameter / 2.0, self.width)
    }
    
    /// Generate the inner bore of the spool
    pub fn inner_bore(&self) -> CSG<()> {
        CSG::cylinder(self.inner_diameter / 2.0, self.width)
    }
    
    /// Generate the hub of the spool
    pub fn hub(&self) -> CSG<()> {
        CSG::cylinder(self.hub_diameter / 2.0, self.hub_width)
            .translate(0.0, 0.0, (self.width - self.hub_width) / 2.0)
    }
    
    /// Generate the full spool model
    pub fn assemble(&self) -> CSG<()> {
        let mut spool = self.outer_shell();
        let bore = self.inner_bore();
        let hub = self.hub();
        
        spool = spool.subtract(&bore);
        spool = spool.union(&hub);
        
        spool
    }
}

