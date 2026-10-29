pub struct DistanceExercise;

impl DistanceExercise {
    pub fn distance_2d(a: (f64, f64), b: (f64, f64)) -> f64 {
        let (x1, y1) = a;
        let (x2, y2) = b;
        ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt()
    }

    pub fn distance_3d(a: (f64, f64, f64), b: (f64, f64, f64)) -> f64 {
        let (x1, y1, z1) = a;
        let (x2, y2, z2) = b;
        ((x2 - x1).powi(2) + (y2 - y1).powi(2) + (z2 - z1).powi(2)).sqrt()
    }
}
