use csgrs::CSG;
use nalgebra::{Point3, Vector3};

/// Representation of an antenna module
#[derive(Debug, Clone)]
pub struct Antenna {
    pub name: String,
    pub base_diameter: f64,
    pub base_height: f64,
    pub rod_diameter: f64,
    pub rod_length: f64,
    pub mounting_hole_diameter: f64,
    pub mounting_hole_spacing: f64,
}

impl Antenna {
    /// Generate the base of the antenna
    pub fn base(&self) -> CSG<()> {
        CSG::cylinder(self.base_diameter / 2.0, self.base_height)
    }
    
    /// Generate the rod of the antenna
    pub fn rod(&self) -> CSG<()> {
        CSG::cylinder(self.rod_diameter / 2.0, self.rod_length)
            .translate(0.0, 0.0, self.base_height)
    }
    
    /// Generate the mounting holes
    pub fn mounting_holes(&self) -> CSG<()> {
        let mut holes = CSG::new();
        let offsets = [
            (-self.mounting_hole_spacing / 2.0, -self.mounting_hole_spacing / 2.0),
            (self.mounting_hole_spacing / 2.0, -self.mounting_hole_spacing / 2.0),
            (self.mounting_hole_spacing / 2.0, self.mounting_hole_spacing / 2.0),
            (-self.mounting_hole_spacing / 2.0, self.mounting_hole_spacing / 2.0),
        ];
        
        for &(x, y) in &offsets {
            let hole = CSG::cylinder(self.mounting_hole_diameter / 2.0, self.base_height)
                .translate(x, y, 0.0);
            holes = holes.union(&hole);
        }
        
        holes
    }
    
    /// Generate the full antenna model
    pub fn assemble(&self) -> CSG<()> {
        let mut antenna = self.base();
        let rod = self.rod();
        let holes = self.mounting_holes();
        
        antenna = antenna.union(&rod);
        antenna = antenna.subtract(&holes);
        
        antenna
    }
}

