use csgrs::csg::CSG;
use csgrs::vertex::Vertex;
use csgrs::polygon::Polygon;
use nalgebra::{Point3, Vector3};

/// Representation of an aluminum extrusion
#[derive(Debug, Clone)]
pub struct Extrusion {
    pub name: String,
    pub width: f64,
    pub height: f64,
    pub center_hole_diameter: f64,
    pub corner_hole_diameter: f64,
    pub channel_width: f64,
    pub internal_channel_width: f64,
    pub tab_thickness: f64,
    pub spar_thickness: f64,
    pub fillet_radius: f64,
}

impl Extrusion {
    /// Generate a cross-section of the extrusion
    pub fn cross_section(&self) -> CSG<()> {
        let vertices = vec![
            Vertex::new(Point3::new(-self.width / 2.0, -self.height / 2.0, 0.0), Vector3::z()),
            Vertex::new(Point3::new(self.width / 2.0, -self.height / 2.0, 0.0), Vector3::z()),
            Vertex::new(Point3::new(self.width / 2.0, self.height / 2.0, 0.0), Vector3::z()),
            Vertex::new(Point3::new(-self.width / 2.0, self.height / 2.0, 0.0), Vector3::z()),
        ];
        let polygon = Polygon::new(vertices, false, None);
        CSG::from_polygons(vec![polygon])
    }

    /// Create a full 3D extrusion with holes
    pub fn extrude(&self, length: f64) -> CSG<()> {
        let mut extrusion = self.cross_section().extrude(length);
        
        // Add center hole if specified
        if self.center_hole_diameter > 0.0 {
            let hole = CSG::cylinder(self.center_hole_diameter / 2.0, length);
            extrusion = extrusion.subtract(&hole);
        }
        
        // Add corner holes if specified
        if self.corner_hole_diameter > 0.0 {
            let offsets = [
                (-self.width / 2.0, -self.height / 2.0),
                (self.width / 2.0, -self.height / 2.0),
                (self.width / 2.0, self.height / 2.0),
                (-self.width / 2.0, self.height / 2.0),
            ];
            
            for &(x, y) in &offsets {
                let hole = CSG::cylinder(self.corner_hole_diameter / 2.0, length).translate(Vector3::new(x, y, 0.0));
                extrusion = extrusion.subtract(&hole);
            }
        }
        
        extrusion
    }
}
