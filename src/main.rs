mod modules {
    pub mod engine;
}

use crate::modules::engine::{Engine, Disposition};

fn main() {

    let cylinders: i8 = 8;
    let cylinder_diameter: f32 = 99.5;
    let cylinder_stroke: f32 = 90.9;
    let disposition = Disposition::V;
    let name = "HEMI V8".to_string();


    let engine = Engine::new(
        cylinders,
        cylinder_diameter,
        cylinder_stroke,
        disposition,
        name
    );

    println!(
        "Engine created: {} cylinders, cylinder diameter: {}, piston stroke: {}, disposition: {:?}, name: {:?}",
        engine.cylinders,
        engine.cylinder_diameter,
        engine.cylinder_stroke,
        engine.disposition,
        engine.name
    );

}
