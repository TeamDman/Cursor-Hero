pub mod f32 {
    use glam::f32::Vec2;
    pub trait NegativeY {
        fn neg_y(&self) -> Vec2;
    }
    
    impl NegativeY for Vec2 {
        fn neg_y(&self) -> Vec2 {
            Vec2::new(self.x, -self.y)
        }
    }

    impl NegativeY for bevy::math::Vec2 {
        fn neg_y(&self) -> Vec2 {
            Vec2::new(self.x, -self.y)
        }
    }
}

pub mod bevy {
    use bevy::prelude::Vec2;
    pub trait NegativeY {
        fn neg_y(&self) -> Vec2;
    }
    impl NegativeY for Vec2 {
        fn neg_y(&self) -> Vec2 {
            Vec2::new(self.x, -self.y)
        }
    }
}