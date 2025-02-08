use csgrs::CSG;

/// Representation of a Door Hinge
#[derive(Debug, Clone)]
pub struct DoorHinge {
    pub leaf_width: f64,
    pub leaf_height: f64,
    pub thickness: f64,
    pub pin_diameter: f64,
    pub hole_diameter: f64,
    pub hole_spacing: f64,
    pub hole_count: usize,
}

impl DoorHinge {
    /// Generate the hinge leaves
    pub fn leaves(&self) -> CSG<()> {
        let leaf = CSG::box_shape(self.leaf_width, self.leaf_height, self.thickness);
        leaf.union(&leaf.translate(0.0, 0.0, self.thickness * 2.0))
    }

    /// Generate the pin
    pub fn pin(&self) -> CSG<()> {
        CSG::cylinder(self.pin_diameter / 2.0, self.thickness * 3.0)
    }

    /// Generate the mounting holes
    pub fn holes(&self) -> CSG<()> {
        let hole = CSG::cylinder(self.hole_diameter / 2.0, self.thickness + 1.0);
        let mut hole_array = CSG::empty();
        let start_x = -((self.hole_count as f64 - 1.0) / 2.0) * self.hole_spacing;

        for i in 0..self.hole_count {
            let x_offset = start_x + (i as f64) * self.hole_spacing;
            hole_array = hole_array.union(&hole.translate(x_offset, 0.0, 0.0));
        }

        hole_array
    }

    /// Assemble the complete door hinge
    pub fn assemble(&self) -> CSG<()> {
        self.leaves().union(&self.pin()).difference(&self.holes())
    }
}

