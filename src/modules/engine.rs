    #[derive(Debug)]
    pub enum Disposition {
        Inline,
        V,
        Boxer,
    }

    pub struct Engine {
        pub cylinders: i8,
        pub disposition: Disposition,
        pub name: String
    }

    impl Engine {
        pub fn new(cylinders: i8, disposition: Disposition, name: String) -> Self {
            Self {
                cylinders,
                disposition,
                name
            }

        }
    }
