mod modules {
    pub mod engine;
}

use crate::modules::engine::{Engine, Disposition};

fn main() {

    let cylinders: u8 = 8;
    let cylinder_diameter: f32 = 99.5;
    let cylinder_stroke: f32 = 90.9;
    let disposition = Disposition::V;
    let name = "HEMI V8".to_string();
    let engine_displacement = Engine::engine_displacement(cylinders, cylinder_diameter, cylinder_stroke);

    let engine = Engine::new(
        cylinders,
        cylinder_diameter,
        cylinder_stroke,
        disposition,
        engine_displacement,
        name
    );

    println!(
        "Engine created: {} cylinders, cylinder diameter: {}, piston stroke: {}, disposition: {:?}, name: {:?}, engine displacement: {} L",
        engine.cylinders,
        engine.cylinder_diameter,
        engine.cylinder_stroke,
        engine.disposition,
        engine.name,
        engine.engine_displacement
    );

}
