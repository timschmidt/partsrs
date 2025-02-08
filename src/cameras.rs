use csgrs::CSG;
use nalgebra::{Point3, Vector3};

/// Representation of a camera module
#[derive(Debug, Clone)]
pub struct Camera {
    pub name: String,
    pub body_width: f64,
    pub body_height: f64,
    pub body_depth: f64,
    pub lens_diameter: f64,
    pub lens_depth: f64,
    pub mounting_hole_diameter: f64,
    pub mounting_hole_spacing: f64,
}

impl Camera {
    /// Generate the rectangular body of the camera
    pub fn body(&self) -> CSG<()> {
        CSG::prism(self.body_width, self.body_height, self.body_depth)
    }
    
    /// Generate the lens of the camera
    pub fn lens(&self) -> CSG<()> {
        CSG::cylinder_z(self.lens_diameter / 2.0, self.lens_depth)
            .translate(Vector3::new(0.0, 0.0, self.body_depth))
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
            let hole = CSG::cylinder_z(self.mounting_hole_diameter / 2.0, self.body_depth)
                .translate(Vector3::new(x, y, 0.0));
            holes = holes.union(&hole);
        }
        
        holes
    }
    
    /// Generate the full camera model
    pub fn assemble(&self) -> CSG<()> {
        let mut camera = self.body();
        let lens = self.lens();
        let holes = self.mounting_holes();
        
        camera = camera.union(&lens);
        camera = camera.subtract(&holes);
        
        camera
    }
}

