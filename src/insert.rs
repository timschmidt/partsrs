use csgrs::CSG;

/// Representation of an Insert module
#[derive(Debug, Clone)]
pub struct Insert {
    pub name: String,
    pub outer_diameter: f64,
    pub inner_diameter: f64,
    pub length: f64,
    pub thread_pitch: f64,
}

impl Insert {
    /// Generate the outer shell of the insert
    pub fn outer_shell(&self) -> CSG<()> {
        CSG::cylinder(self.outer_diameter / 2.0, self.length)
    }
    
    /// Generate the inner bore of the insert
    pub fn inner_bore(&self) -> CSG<()> {
        CSG::cylinder(self.inner_diameter / 2.0, self.length)
    }
    
    /// Generate the threading of the insert
    pub fn threading(&self) -> CSG<()> {
        // Simplified representation of threading using a helical pattern
        CSG::helix(self.inner_diameter / 2.0, self.length, self.thread_pitch)
    }
    
    /// Generate the full insert model
    pub fn assemble(&self) -> CSG<()> {
        let mut insert = self.outer_shell();
        let bore = self.inner_bore();
        let threads = self.threading();
        
        insert = insert.subtract(&bore);
        insert = insert.union(&threads);
        
        insert
    }
}

