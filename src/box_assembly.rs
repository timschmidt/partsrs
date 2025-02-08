use csgrs::CSG;

/// Representation of a Box Assembly
#[derive(Debug, Clone)]
pub struct BoxAssembly {
    pub width: f64,
    pub depth: f64,
    pub height: f64,
    pub sheet_thickness: f64,
}

impl BoxAssembly {
    /// Generate the box panels
    pub fn panels(&self) -> CSG<()> {
        let base = CSG::box_shape(self.width, self.depth, self.sheet_thickness);
        let top = base.translate(0.0, 0.0, self.height);

        let side = CSG::box_shape(self.sheet_thickness, self.depth, self.height)
            .translate(self.width / 2.0, 0.0, self.height / 2.0)
            .union(&CSG::box_shape(self.sheet_thickness, self.depth, self.height)
                .translate(-self.width / 2.0, 0.0, self.height / 2.0));

        let front_back = CSG::box_shape(self.width, self.sheet_thickness, self.height)
            .translate(0.0, self.depth / 2.0, self.height / 2.0)
            .union(&CSG::box_shape(self.width, self.sheet_thickness, self.height)
                .translate(0.0, -self.depth / 2.0, self.height / 2.0));

        base.union(&top).union(&side).union(&front_back)
    }

    /// Assemble the complete box
    pub fn assemble(&self) -> CSG<()> {
        self.panels()
    }
}

