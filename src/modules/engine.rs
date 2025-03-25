    const PI :f32 = 3.1416;

    pub fn cm_to_lt(num: f32) -> f32 {
        let num = (num / 100000.0).ceil() * 0.1;
        (num * 10.0).trunc() / 10.0
    }

    
    #[derive(Debug)]
    pub enum Disposition {
        Inline,
        V,
        Boxer,
    }

    pub struct Engine {
        pub cylinders: u8,
        pub cylinder_diameter: f32,
        pub cylinder_stroke: f32,
        pub disposition: Disposition,
        pub engine_displacement: f32,
        pub name: String
    }

    impl Engine {
        pub fn new(cylinders: u8, cylinder_diameter: f32, cylinder_stroke: f32, disposition: Disposition, engine_displacement: f32, name: String) -> Self {
            Self {
                cylinders,
                cylinder_diameter,
                cylinder_stroke,
                disposition,
                engine_displacement,
                name
            }
        }
        
        pub fn engine_displacement(cylinders: u8, cylinder_diameter: f32, cylinder_stroke: f32) -> f32 {

            let cylinders: f32 = cylinders as f32;
            let engine_displacement = (PI / 4 as f32) * (cylinder_diameter * cylinder_diameter) * cylinder_stroke * cylinders;
            return cm_to_lt(engine_displacement);
        }
    }
