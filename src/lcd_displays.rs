use csgrs::CSG;
use nalgebra::{Point3, Vector3};

/// Representation of an LCD display module
#[derive(Debug, Clone)]
pub struct LCDDisplay {
    pub name: String,
    pub screen_width: f64,
    pub screen_height: f64,
    pub screen_depth: f64,
    pub bezel_width: f64,
    pub bezel_height: f64,
    pub bezel_depth: f64,
    pub mounting_hole_diameter: f64,
    pub mounting_hole_spacing: f64,
}

impl LCDDisplay {
    /// Generate the screen of the LCD display
    pub fn screen(&self) -> CSG<()> {
        CSG::box_shape(self.screen_width, self.screen_height, self.screen_depth)
    }
    
    /// Generate the bezel of the LCD display
    pub fn bezel(&self) -> CSG<()> {
        CSG::box_shape(self.bezel_width, self.bezel_height, self.bezel_depth)
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
            let hole = CSG::cylinder(self.mounting_hole_diameter / 2.0, self.bezel_depth)
                .translate(x, y, 0.0);
            holes = holes.union(&hole);
        }
        
        holes
    }
    
    /// Generate the full LCD display model
    pub fn assemble(&self) -> CSG<()> {
        let mut display = self.bezel();
        let screen = self.screen().translate(0.0, 0.0, self.bezel_depth - self.screen_depth);
        let holes = self.mounting_holes();
        
        display = display.union(&screen);
        display = display.subtract(&holes);
        
        display
    }
}

