use csgrs::CSG;

/// Representation of a Heat Pipe module
#[derive(Debug, Clone)]
pub struct HeatPipe {
    pub name: String,
    pub outer_diameter: f64,
    pub inner_diameter: f64,
    pub length: f64,
    pub fin_diameter: f64,
    pub fin_thickness: f64,
    pub fin_count: u32,
}

impl HeatPipe {
    /// Generate the outer shell of the heat pipe
    pub fn outer_shell(&self) -> CSG<()> {
        CSG::cylinder(self.outer_diameter / 2.0, self.length)
    }
    
    /// Generate the inner bore of the heat pipe
    pub fn inner_bore(&self) -> CSG<()> {
        CSG::cylinder(self.inner_diameter / 2.0, self.length)
    }
    
    /// Generate the cooling fins
    pub fn fins(&self) -> CSG<()> {
        let mut fins = CSG::new();
        for i in 0..self.fin_count {
            let z_offset = (i as f64) * (self.length / self.fin_count as f64);
            let fin = CSG::cylinder(self.fin_diameter / 2.0, self.fin_thickness)
                .translate(Vector3::new(0.0, 0.0, z_offset));
            fins = fins.union(&fin);
        }
        fins
    }
    
    /// Generate the full heat pipe model
    pub fn assemble(&self) -> CSG<()> {
        let mut pipe = self.outer_shell();
        let bore = self.inner_bore();
        let fins = self.fins();
        
        pipe = pipe.subtract(&bore);
        pipe = pipe.union(&fins);
        
        pipe
    }
}

