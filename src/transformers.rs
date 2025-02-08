use csgrs::CSG;
use nalgebra::{Point3, Vector3};

/// Representation of a transformer module
#[derive(Debug, Clone)]
pub struct Transformer {
    pub name: String,
    pub core_width: f64,
    pub core_height: f64,
    pub core_depth: f64,
    pub winding_diameter: f64,
    pub winding_height: f64,
    pub mounting_hole_diameter: f64,
    pub mounting_hole_spacing: f64,
}

impl Transformer {
    /// Generate the rectangular core of the transformer
    pub fn core(&self) -> CSG<()> {
        CSG::prism(self.core_width, self.core_height, self.core_depth)
    }
    
    /// Generate the windings of the transformer
    pub fn windings(&self) -> CSG<()> {
        CSG::cylinder_z(self.winding_diameter / 2.0, self.winding_height)
            .translate(Vector3::new(0.0, 0.0, self.core_depth / 2.0))
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
            let hole = CSG::cylinder_z(self.mounting_hole_diameter / 2.0, self.core_depth)
                .translate(Vector3::new(x, y, 0.0));
            holes = holes.union(&hole);
        }
        
        holes
    }
    
    /// Generate the full transformer model
    pub fn assemble(&self) -> CSG<()> {
        let mut transformer = self.core();
        let windings = self.windings();
        let holes = self.mounting_holes();
        
        transformer = transformer.union(&windings);
        transformer = transformer.subtract(&holes);
        
        transformer
    }
}

