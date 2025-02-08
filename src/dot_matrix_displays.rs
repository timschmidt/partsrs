use csgrs::CSG;
use nalgebra::{Point3, Vector3};

/// Representation of a dot matrix display module
#[derive(Debug, Clone)]
pub struct DotMatrixDisplay {
    pub name: String,
    pub pixel_diameter: f64,
    pub pixel_spacing: f64,
    pub rows: u32,
    pub columns: u32,
    pub panel_width: f64,
    pub panel_height: f64,
    pub panel_depth: f64,
}

impl DotMatrixDisplay {
    /// Generate a single pixel of the display
    pub fn pixel(&self) -> CSG<()> {
        CSG::cylinder(self.pixel_diameter / 2.0, self.panel_depth)
    }
    
    /// Generate the panel of the display
    pub fn panel(&self) -> CSG<()> {
        CSG::prism(self.panel_width, self.panel_height, self.panel_depth)
    }
    
    /// Generate the full dot matrix display model
    pub fn assemble(&self) -> CSG<()> {
        let mut display = self.panel();
        let start_x = -((self.columns as f64 - 1.0) * self.pixel_spacing) / 2.0;
        let start_y = -((self.rows as f64 - 1.0) * self.pixel_spacing) / 2.0;
        
        for row in 0..self.rows {
            for col in 0..self.columns {
                let x_offset = start_x + col as f64 * self.pixel_spacing;
                let y_offset = start_y + row as f64 * self.pixel_spacing;
                let pixel = self.pixel().translate(Vector3::new(x_offset, y_offset, self.panel_depth / 2.0));
                display = display.union(&pixel);
            }
        }
        
        display
    }
}

