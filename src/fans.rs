use csgrs::CSG;
use nalgebra::{Point3, Vector3};

/// Representation of a cooling fan
#[derive(Debug, Clone)]
pub struct Fan {
    pub name: String,
    pub frame_width: f64,
    pub frame_height: f64,
    pub frame_depth: f64,
    pub blade_diameter: f64,
    pub hub_diameter: f64,
    pub blade_count: u32,
    pub mounting_hole_diameter: f64,
    pub mounting_hole_spacing: f64,
}

impl Fan {
    /// Generate the frame of the fan
    pub fn frame(&self) -> CSG<()> {
        CSG::box_shape(self.frame_width, self.frame_height, self.frame_depth)
    }
    
    /// Generate the blades of the fan
    pub fn blades(&self) -> CSG<()> {
        let mut blades = CSG::new();
        let angle_step = 360.0 / self.blade_count as f64;
        
        for i in 0..self.blade_count {
            let angle = i as f64 * angle_step;
            let blade = CSG::cylinder(self.blade_diameter / 2.0, self.frame_depth / 2.0)
                .rotate(0.0, 0.0, angle)
                .translate(0.0, 0.0, self.frame_depth / 2.0);
            blades = blades.union(&blade);
        }
        
        blades
    }
    
    /// Generate the hub of the fan
    pub fn hub(&self) -> CSG<()> {
        CSG::cylinder(self.hub_diameter / 2.0, self.frame_depth)
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
            let hole = CSG::cylinder(self.mounting_hole_diameter / 2.0, self.frame_depth)
                .translate(x, y, 0.0);
            holes = holes.union(&hole);
        }
        
        holes
    }
    
    /// Generate the full fan model
    pub fn assemble(&self) -> CSG<()> {
        let mut fan = self.frame();
        let blades = self.blades();
        let hub = self.hub();
        let holes = self.mounting_holes();
        
        fan = fan.union(&blades);
        fan = fan.union(&hub);
        fan = fan.subtract(&holes);
        
        fan
    }
}

