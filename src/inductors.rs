use csgrs::CSG;
use nalgebra::{Point3, Vector3};

/// Representation of an inductor module
#[derive(Debug, Clone)]
pub struct Inductor {
    pub name: String,
    pub core_diameter: f64,
    pub core_height: f64,
    pub winding_diameter: f64,
    pub winding_spacing: f64,
    pub winding_count: u32,
}

impl Inductor {
    /// Generate the cylindrical core of the inductor
    pub fn core(&self) -> CSG<()> {
        CSG::cylinder_z(self.core_diameter / 2.0, self.core_height)
    }
    
    /// Generate the windings of the inductor
    pub fn windings(&self) -> CSG<()> {
        let mut windings = CSG::new();
        let start_z = -self.core_height / 2.0;
        
        for i in 0..self.winding_count {
            let z_offset = start_z + i as f64 * self.winding_spacing;
            let winding = CSG::torus(self.winding_diameter / 2.0, self.winding_spacing / 2.0)
                .translate(Vector3::new(0.0, 0.0, z_offset));
            windings = windings.union(&winding);
        }
        
        windings
    }
    
    /// Generate the full inductor model
    pub fn assemble(&self) -> CSG<()> {
        let mut inductor = self.core();
        let windings = self.windings();
        
        inductor = inductor.union(&windings);
        
        inductor
    }
}

