use csgrs::CSG;

/// Representation of a Gear Motor module
#[derive(Debug, Clone)]
pub struct GearMotor {
    pub name: String,
    pub body_diameter: f64,
    pub body_length: f64,
    pub shaft_diameter: f64,
    pub shaft_length: f64,
    pub gear_diameter: f64,
    pub gear_width: f64,
    pub mounting_hole_diameter: f64,
    pub mounting_hole_spacing: f64,
}

impl GearMotor {
    /// Generate the cylindrical body of the gear motor
    pub fn body(&self) -> CSG<()> {
        CSG::cylinder(self.body_diameter / 2.0, self.body_length)
    }
    
    /// Generate the shaft of the gear motor
    pub fn shaft(&self) -> CSG<()> {
        CSG::cylinder(self.shaft_diameter / 2.0, self.shaft_length)
            .translate(Vector3::new(0.0, 0.0, self.body_length))
    }
    
    /// Generate the gear of the gear motor
    pub fn gear(&self) -> CSG<()> {
        CSG::cylinder(self.gear_diameter / 2.0, self.gear_width)
            .translate(Vector3::new(0.0, 0.0, self.body_length + self.shaft_length))
    }
    
    /// Generate the mounting holes of the gear motor
    pub fn mounting_holes(&self) -> CSG<()> {
        let mut holes = CSG::new();
        let offsets = [
            (-self.mounting_hole_spacing / 2.0, 0.0),
            (self.mounting_hole_spacing / 2.0, 0.0),
        ];
        
        for &(x, y) in &offsets {
            let hole = CSG::cylinder(self.mounting_hole_diameter / 2.0, self.body_length)
                .translate(Vector3::new(x, y, 0.0));
            holes = holes.union(&hole);
        }
        
        holes
    }
    
    /// Generate the full gear motor model
    pub fn assemble(&self) -> CSG<()> {
        let mut motor = self.body();
        let shaft = self.shaft();
        let gear = self.gear();
        let holes = self.mounting_holes();
        
        motor = motor.union(&shaft);
        motor = motor.union(&gear);
        motor = motor.subtract(&holes);
        
        motor
    }
}

