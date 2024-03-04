pub trait Lerp {
    fn lerp(self, open: f32) -> f32;
}

impl Lerp for (f32, f32) {
    fn lerp(self, open: f32) -> f32 {
        let (start, end) = self;
        start + (end - start) * open.clamp(0.0, 1.0)
    }
}
