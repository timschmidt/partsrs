use partsrs::extrusion::E2020;

fn main() {
    let e2020_2d = E2020.cross_section_2d(false);
    let e2020_3d = E2020.extrude_3d(100.0, true, false);
}
