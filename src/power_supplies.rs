use csgrs::CSG;
use nalgebra::{Point3, Vector3};

/// Representation of a power supply module
#[derive(Debug, Clone)]
pub struct PowerSupply {
    pub name: String,
    pub body_width: f64,
    pub body_height: f64,
    pub body_depth: f64,
    pub vent_hole_diameter: f64,
    pub vent_hole_spacing: f64,
    pub mounting_hole_diameter: f64,
    pub mounting_hole_spacing: f64,
}

impl PowerSupply {
    /// Generate the rectangular body of the power supply
    pub fn body(&self) -> CSG<()> {
        CSG::prism(self.body_width, self.body_height, self.body_depth)
    }
    
    /// Generate the vent holes of the power supply
    pub fn vent_holes(&self) -> CSG<()> {
        let mut holes = CSG::new();
        let num_holes = (self.body_width / self.vent_hole_spacing).floor() as i32;
        
        for i in 0..num_holes {
            let x_offset = (i as f64 * self.vent_hole_spacing) - (self.body_width / 2.0);
            let hole = CSG::cylinder_z(self.vent_hole_diameter / 2.0, self.body_height)
                .translate(Vector3::new(x_offset, 0.0, self.body_depth / 2.0));
            holes = holes.union(&hole);
        }
        
        holes
    }
    
    /// Generate the mounting holes of the power supply
    pub fn mounting_holes(&self) -> CSG<()> {
        let mut holes = CSG::new();
        let offsets = [
            (-self.mounting_hole_spacing / 2.0, -self.mounting_hole_spacing / 2.0),
            (self.mounting_hole_spacing / 2.0, -self.mounting_hole_spacing / 2.0),
            (self.mounting_hole_spacing / 2.0, self.mounting_hole_spacing / 2.0),
            (-self.mounting_hole_spacing / 2.0, self.mounting_hole_spacing / 2.0),
        ];
        
        for &(x, y) in &offsets {
            let hole = CSG::cylinder_z(self.mounting_hole_diameter / 2.0, self.body_depth)
                .translate(Vector3::new(x, y, 0.0));
            holes = holes.union(&hole);
        }
        
        holes
    }
    
    /// Generate the full power supply model
    pub fn assemble(&self) -> CSG<()> {
        let mut power_supply = self.body();
        let vent_holes = self.vent_holes();
        let mounting_holes = self.mounting_holes();
        
        power_supply = power_supply.subtract(&vent_holes);
        power_supply = power_supply.subtract(&mounting_holes);
        
        power_supply
    }
}

