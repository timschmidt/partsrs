use csgrs::CSG;
use nalgebra::{Point3, Vector3};

/// Representation of a microphone
#[derive(Debug, Clone)]
pub struct Microphone {
    pub name: String,
    pub body_diameter: f64,
    pub body_height: f64,
    pub grille_diameter: f64,
    pub grille_height: f64,
    pub mounting_hole_diameter: f64,
    pub mounting_hole_spacing: f64,
}

impl Microphone {
    /// Generate the cylindrical body of the microphone
    pub fn body(&self) -> CSG<()> {
        CSG::cylinder_z(self.body_diameter / 2.0, self.body_height)
    }
    
    /// Generate the grille of the microphone
    pub fn grille(&self) -> CSG<()> {
        CSG::cylinder_z(self.grille_diameter / 2.0, self.grille_height)
            .translate(Vector3::new(0.0, 0.0, self.body_height))
    }
    
    /// Generate the mounting holes of the microphone
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
    
    /// Generate the full microphone model
    pub fn assemble(&self) -> CSG<()> {
        let mut microphone = self.body();
        let grille = self.grille();
        let holes = self.mounting_holes();
        
        microphone = microphone.union(&grille);
        microphone = microphone.subtract(&holes);
        
        microphone
    }
}

