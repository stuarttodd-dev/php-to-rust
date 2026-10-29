mod distance;
mod employee;
mod point;
mod rgb;

fn main() {
    run_all_checks();
}

fn run_all_checks() {
    let d2 = distance::DistanceExercise::distance_2d((0.0, 0.0), (3.0, 4.0));
    assert!((d2 - 5.0).abs() < 1e-9, "2D: expected 5, got {}", d2);
    let d3 = distance::DistanceExercise::distance_3d((0.0, 0.0, 0.0), (2.0, 3.0, 6.0));
    assert!((d3 - 7.0).abs() < 1e-9, "3D: expected 7, got {}", d3);

    assert_eq!(rgb::RgbTupleExercise::to_hex((255, 0, 0)), "#ff0000");

    let (id, name, salary) = employee::EmployeeTupleExercise::get_employee();
    println!("ID: {}, Name: {}, Salary: {}", id, name, salary);

    let p = point::PointTranslationExercise::translate((1.0, 2.0, 3.0), (10.0, -1.0, 0.5));
    assert!((p.0 - 11.0).abs() < 1e-9 && (p.1 - 1.0).abs() < 1e-9 && (p.2 - 3.5).abs() < 1e-9);

    println!("All checks passed.");
}
