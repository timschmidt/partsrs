use csgrs::CSG;
use nalgebra::{Point3, Vector3};

/// Representation of a seven-segment display module
#[derive(Debug, Clone)]
pub struct SevenSegmentDisplay {
    pub name: String,
    pub digit_width: f64,
    pub digit_height: f64,
    pub digit_depth: f64,
    pub segment_width: f64,
    pub segment_height: f64,
    pub segment_depth: f64,
    pub digit_spacing: f64,
    pub num_digits: u32,
}

impl SevenSegmentDisplay {
    /// Generate a single digit of the display
    pub fn digit(&self) -> CSG<()> {
        CSG::prism(self.digit_width, self.digit_height, self.digit_depth)
    }
    
    /// Generate the segments within a digit
    pub fn segments(&self) -> CSG<()> {
        let mut segments = CSG::new();
        let positions = [
            (-self.segment_width, self.segment_height / 2.0),
            (self.segment_width, self.segment_height / 2.0),
            (-self.segment_width, -self.segment_height / 2.0),
            (self.segment_width, -self.segment_height / 2.0),
        ];
        
        for &(x, y) in &positions {
            let segment = CSG::prism(self.segment_width, self.segment_height, self.segment_depth)
                .translate(x, y, self.digit_depth / 2.0);
            segments = segments.union(&segment);
        }
        
        segments
    }
    
    /// Generate the full seven-segment display model
    pub fn assemble(&self) -> CSG<()> {
        let mut display = CSG::new();
        let start_x = -((self.num_digits as f64 - 1.0) * self.digit_spacing) / 2.0;
        
        for i in 0..self.num_digits {
            let x_offset = start_x + i as f64 * self.digit_spacing;
            let digit = self.digit().translate(x_offset, 0.0, 0.0);
            let segments = self.segments().translate(x_offset, 0.0, 0.0);
            
            display = display.union(&digit);
            display = display.union(&segments);
        }
        
        display
    }
}

