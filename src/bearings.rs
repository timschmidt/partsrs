use csgrs::CSG;
use nalgebra::{Point3, Vector3};

/// Representation of a linear bearing
#[derive(Debug, Clone)]
pub struct Bearing {
    pub name: String,
    pub outer_diameter: f64,
    pub inner_diameter: f64,
    pub length: f64,
    pub groove_diameter: Option<f64>,
    pub groove_spacing: Option<f64>,
}

impl Bearing {
    /// Generate the outer cylindrical body of the bearing
    pub fn outer_body(&self) -> CSG<()> {
        CSG::cylinder(self.outer_diameter / 2.0, self.length)
    }
    
    /// Generate the inner hole of the bearing
    pub fn hole(&self) -> CSG<()> {
        CSG::cylinder(self.inner_diameter / 2.0, self.length)
    }
    
    /// Generate the grooves if specified
    pub fn grooves(&self) -> Option<CSG<()>> {
        if let (Some(diameter), Some(spacing)) = (self.groove_diameter, self.groove_spacing) {
            let mut grooves = CSG::new();
            let num_grooves = (self.length / spacing).floor() as i32;
            for i in 0..num_grooves {
                let groove = CSG::cylinder(diameter / 2.0, spacing / 4.0)
                    .translate(0.0, 0.0, i as f64 * spacing);
                grooves = grooves.union(&groove);
            }
            Some(grooves)
        } else {
            None
        }
    }
    
    /// Generate the full bearing model
    pub fn assemble(&self) -> CSG<()> {
        let mut bearing = self.outer_body();
        let hole = self.hole();
        bearing = bearing.subtract(&hole);
        
        if let Some(grooves) = self.grooves() {
            bearing = bearing.subtract(&grooves);
        }
        
        bearing
    }
}

