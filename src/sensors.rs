use csgrs::CSG;
use nalgebra::{Point3, Vector3};

/// Representation of a sensor
#[derive(Debug, Clone)]
pub struct Sensor {
    pub name: String,
    pub body_width: f64,
    pub body_height: f64,
    pub body_depth: f64,
    pub lens_diameter: f64,
    pub lens_depth: f64,
    pub mounting_hole_diameter: f64,
    pub mounting_hole_spacing: f64,
}

impl Sensor {
    /// Generate the body of the sensor
    pub fn body(&self) -> CSG<()> {
        CSG::box_shape(self.body_width, self.body_height, self.body_depth)
    }
    
    /// Generate the lens of the sensor
    pub fn lens(&self) -> CSG<()> {
        CSG::cylinder(self.lens_diameter / 2.0, self.lens_depth)
            .translate(0.0, 0.0, self.body_depth)
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
            let hole = CSG::cylinder(self.mounting_hole_diameter / 2.0, self.body_depth)
                .translate(x, y, 0.0);
            holes = holes.union(&hole);
        }
        
        holes
    }
    
    /// Generate the full sensor model
    pub fn assemble(&self) -> CSG<()> {
        let mut sensor = self.body();
        let lens = self.lens();
        let holes = self.mounting_holes();
        
        sensor = sensor.union(&lens);
        sensor = sensor.subtract(&holes);
        
        sensor
    }
}

