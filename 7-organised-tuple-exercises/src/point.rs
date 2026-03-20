pub struct PointTranslationExercise;

impl PointTranslationExercise {
    pub fn translate(point: (f64, f64, f64), delta: (f64, f64, f64)) -> (f64, f64, f64) {
        (point.0 + delta.0, point.1 + delta.1, point.2 + delta.2)
    }
}
