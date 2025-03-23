    #[derive(Debug)]
    pub enum Disposition {
        Inline,
        V,
        Boxer,
    }

    pub struct Engine {
        pub cylinders: i8,
        pub cylinder_diameter: f32,
        pub cylinder_stroke: f32,
        pub disposition: Disposition,
        pub name: String
    }

    impl Engine {
        pub fn new(cylinders: i8, cylinder_diameter: f32, cylinder_stroke: f32, disposition: Disposition, name: String) -> Self {
            Self {
                cylinders,
                cylinder_diameter,
                cylinder_stroke,
                disposition,
                name
            }

        }
    }
