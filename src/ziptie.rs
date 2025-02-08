use csgrs::CSG;

/// Representation of a Zip Tie module
#[derive(Debug, Clone)]
pub struct ZipTie {
    pub name: String,
    pub length: f64,
    pub width: f64,
    pub thickness: f64,
    pub head_width: f64,
    pub head_height: f64,
    pub head_depth: f64,
}

impl ZipTie {
    /// Generate the strap of the zip tie
    pub fn strap(&self) -> CSG<()> {
        CSG::box_shape(self.length, self.width, self.thickness)
    }
    
    /// Generate the head of the zip tie
    pub fn head(&self) -> CSG<()> {
        CSG::box_shape(self.head_width, self.head_height, self.head_depth)
            .translate(self.length / 2.0 - self.head_depth / 2.0, 0.0, 0.0)
    }
    
    /// Generate the full zip tie model
    pub fn assemble(&self) -> CSG<()> {
        let mut ziptie = self.strap();
        let head = self.head();
        
        ziptie = ziptie.union(&head);
        
        ziptie
    }
}

