use csgrs::CSG;
use nalgebra::{Point3, Vector3};

/// Representation of a large speaker module
#[derive(Debug, Clone)]
pub struct LargeSpeaker {
    pub name: String,
    pub body_diameter: f64,
    pub body_height: f64,
    pub cone_diameter: f64,
    pub cone_depth: f64,
    pub mounting_hole_diameter: f64,
    pub mounting_hole_spacing: f64,
}

impl LargeSpeaker {
    /// Generate the cylindrical body of the speaker
    pub fn body(&self) -> CSG<()> {
        CSG::cylinder_z(self.body_diameter / 2.0, self.body_height)
    }
    
    /// Generate the speaker cone
    pub fn cone(&self) -> CSG<()> {
        CSG::cone(self.cone_diameter / 2.0, self.cone_depth)
            .translate(Vector3::new(0.0, 0.0, self.body_height))
    }
    
    /// Generate the mounting holes of the speaker
    pub fn mounting_holes(&self) -> CSG<()> {
        let mut holes = CSG::new();
        let offsets = [
            (-self.mounting_hole_spacing / 2.0, -self.mounting_hole_spacing / 2.0),
            (self.mounting_hole_spacing / 2.0, -self.mounting_hole_spacing / 2.0),
            (self.mounting_hole_spacing / 2.0, self.mounting_hole_spacing / 2.0),
            (-self.mounting_hole_spacing / 2.0, self.mounting_hole_spacing / 2.0),
        ];
        
        for &(x, y) in &offsets {
            let hole = CSG::cylinder_z(self.mounting_hole_diameter / 2.0, self.body_height)
                .translate(Vector3::new(x, y, 0.0));
            holes = holes.union(&hole);
        }
        
        holes
    }
    
    /// Generate the full large speaker model
    pub fn assemble(&self) -> CSG<()> {
        let mut speaker = self.body();
        let cone = self.cone();
        let holes = self.mounting_holes();
        
        speaker = speaker.union(&cone);
        speaker = speaker.subtract(&holes);
        
        speaker
    }
}

