use csgrs::CSG;

/// Representation of a Cable Strip
#[derive(Debug, Clone)]
pub struct CableStrip {
    pub width: f64,
    pub height: f64,
    pub depth: f64,
    pub slot_width: f64,
    pub slot_depth: f64,
    pub slot_spacing: f64,
    pub slot_count: usize,
}

impl CableStrip {
    /// Generate the main strip body
    pub fn body(&self) -> CSG<()> {
        CSG::prism(self.width, self.height, self.depth)
    }

    /// Generate the slots for the cables
    pub fn slots(&self) -> CSG<()> {
        let slot = CSG::prism(self.slot_width, self.slot_depth, self.height);
        let mut slots = CSG::new();
        let start_x = -((self.slot_count as f64 - 1.0) / 2.0) * self.slot_spacing;

        for i in 0..self.slot_count {
            let x_offset = start_x + (i as f64) * self.slot_spacing;
            slots = slots.union(&slot.translate(Vector3::new(x_offset, 0.0, self.height / 2.0)));
        }

        slots
    }

    /// Assemble the complete cable strip
    pub fn assemble(&self) -> CSG<()> {
        self.body().difference(&self.slots())
    }
}

