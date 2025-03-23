mod modules {
    pub mod engine;
}

use crate::modules::engine::{Engine, Disposition};

fn main() {

    let cylinders: i8 = 8;
    let disposition = Disposition::V;
    let name = "HEMI V8".to_string();


    let engine = Engine::new(
        cylinders,
        disposition,
        name
    );

    println!("Engine created: {} cylinders, disposition: {:?}, name: {}", engine.cylinders, engine.disposition, engine.name);
}
