use csgrs::CSG;

/// Representation of a BLDC (Brushless DC) Motor module
#[derive(Debug, Clone)]
pub struct BLDCMotor {
    pub name: String,
    pub stator_diameter: f64,
    pub rotor_diameter: f64,
    pub height: f64,
    pub shaft_diameter: f64,
    pub shaft_length: f64,
    pub mounting_hole_diameter: f64,
    pub mounting_hole_spacing: f64,
}

impl BLDCMotor {
    /// Generate the stator of the BLDC motor
    pub fn stator(&self) -> CSG<()> {
        CSG::cylinder(self.stator_diameter / 2.0, self.height)
    }
    
    /// Generate the rotor of the BLDC motor
    pub fn rotor(&self) -> CSG<()> {
        CSG::cylinder(self.rotor_diameter / 2.0, self.height)
    }
    
    /// Generate the shaft of the BLDC motor
    pub fn shaft(&self) -> CSG<()> {
        CSG::cylinder(self.shaft_diameter / 2.0, self.shaft_length)
            .translate(Vector3::new(0.0, 0.0, self.height))
    }
    
    /// Generate the mounting holes of the BLDC motor
    pub fn mounting_holes(&self) -> CSG<()> {
        let mut holes = CSG::new();
        let offsets = [
            (-self.mounting_hole_spacing / 2.0, -self.mounting_hole_spacing / 2.0),
            (self.mounting_hole_spacing / 2.0, -self.mounting_hole_spacing / 2.0),
            (self.mounting_hole_spacing / 2.0, self.mounting_hole_spacing / 2.0),
            (-self.mounting_hole_spacing / 2.0, self.mounting_hole_spacing / 2.0),
        ];
        
        for &(x, y) in &offsets {
            let hole = CSG::cylinder(self.mounting_hole_diameter / 2.0, self.height)
                .translate(Vector3::new(x, y, 0.0));
            holes = holes.union(&hole);
        }
        
        holes
    }
    
    /// Generate the full BLDC motor model
    pub fn assemble(&self) -> CSG<()> {
        let mut motor = self.stator();
        let rotor = self.rotor();
        let shaft = self.shaft();
        let holes = self.mounting_holes();
        
        motor = motor.union(&rotor);
        motor = motor.union(&shaft);
        motor = motor.subtract(&holes);
        
        motor
    }
}

