struct DistanceExercise;

impl DistanceExercise {
    fn distance_2d(a: (f64, f64), b: (f64, f64)) -> f64 {
        let (x1, y1) = a;
        let (x2, y2) = b;
        ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt()
    }

    fn distance_3d(a: (f64, f64, f64), b: (f64, f64, f64)) -> f64 {
        let (x1, y1, z1) = a;
        let (x2, y2, z2) = b;
        ((x2 - x1).powi(2) + (y2 - y1).powi(2) + (z2 - z1).powi(2)).sqrt()
    }
}

struct RgbTupleExercise;

impl RgbTupleExercise {
    fn to_hex(rgb: (u8, u8, u8)) -> String {
        format!("#{:02x}{:02x}{:02x}", rgb.0, rgb.1, rgb.2)
    }
}

struct EmployeeTupleExercise;

impl EmployeeTupleExercise {
    fn get_employee() -> (u32, String, f64) {
        (1001, String::from("Jane Doe"), 85000.0)
    }
}

struct PointTranslationExercise;

impl PointTranslationExercise {
    fn translate(point: (f64, f64, f64), delta: (f64, f64, f64)) -> (f64, f64, f64) {
        (point.0 + delta.0, point.1 + delta.1, point.2 + delta.2)
    }
}

fn main() {
    let d2 = DistanceExercise::distance_2d((0.0, 0.0), (3.0, 4.0));
    assert!((d2 - 5.0).abs() < 1e-9, "2D: expected 5, got {}", d2);
    let d3 = DistanceExercise::distance_3d((0.0, 0.0, 0.0), (2.0, 3.0, 6.0));
    assert!((d3 - 7.0).abs() < 1e-9, "3D: expected 7, got {}", d3);

    assert_eq!(RgbTupleExercise::to_hex((255, 0, 0)), "#ff0000");

    let (id, name, salary) = EmployeeTupleExercise::get_employee();
    println!("ID: {}, Name: {}, Salary: {}", id, name, salary);

    let p = PointTranslationExercise::translate((1.0, 2.0, 3.0), (10.0, -1.0, 0.5));
    assert!((p.0 - 11.0).abs() < 1e-9 && (p.1 - 1.0).abs() < 1e-9 && (p.2 - 3.5).abs() < 1e-9);

    println!("All checks passed.");
}
