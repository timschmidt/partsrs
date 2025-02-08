use csgrs::CSG;
use nalgebra::{Point3, Vector3};

/// Representation of a keypad module
#[derive(Debug, Clone)]
pub struct Keypad {
    pub name: String,
    pub base_width: f64,
    pub base_height: f64,
    pub base_depth: f64,
    pub key_width: f64,
    pub key_height: f64,
    pub key_depth: f64,
    pub rows: u32,
    pub columns: u32,
    pub key_spacing: f64,
}

impl Keypad {
    /// Generate the rectangular base of the keypad
    pub fn base(&self) -> CSG<()> {
        CSG::box_shape(self.base_width, self.base_height, self.base_depth)
    }
    
    /// Generate the keys of the keypad
    pub fn keys(&self) -> CSG<()> {
        let mut keys = CSG::new();
        let start_x = -((self.columns as f64 - 1.0) * self.key_spacing) / 2.0;
        let start_y = -((self.rows as f64 - 1.0) * self.key_spacing) / 2.0;
        
        for row in 0..self.rows {
            for col in 0..self.columns {
                let x_offset = start_x + col as f64 * self.key_spacing;
                let y_offset = start_y + row as f64 * self.key_spacing;
                let key = CSG::box_shape(self.key_width, self.key_height, self.key_depth)
                    .translate(x_offset, y_offset, self.base_depth);
                keys = keys.union(&key);
            }
        }
        
        keys
    }
    
    /// Generate the full keypad model
    pub fn assemble(&self) -> CSG<()> {
        let mut keypad = self.base();
        let keys = self.keys();
        
        keypad = keypad.union(&keys);
        
        keypad
    }
}

