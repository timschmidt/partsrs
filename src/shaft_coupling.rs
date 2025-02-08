use csgrs::CSG;

/// Representation of a Shaft Coupling
#[derive(Debug, Clone)]
pub struct ShaftCoupling {
    pub outer_diameter: f64,
    pub length: f64,
    pub bore_diameter: f64,
}

impl ShaftCoupling {
    /// Generate the outer body of the coupling
    pub fn body(&self) -> CSG<()> {
        CSG::cylinder(self.outer_diameter / 2.0, self.length)
    }

    /// Generate the bore hole
    pub fn bore(&self) -> CSG<()> {
        CSG::cylinder(self.bore_diameter / 2.0, self.length + 1.0)
    }

    /// Assemble the shaft coupling
    pub fn assemble(&self) -> CSG<()> {
        self.body().difference(&self.bore())
    }
}

