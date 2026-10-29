pub struct RgbTupleExercise;

impl RgbTupleExercise {
    pub fn to_hex(rgb: (u8, u8, u8)) -> String {
        format!("#{:02x}{:02x}{:02x}", rgb.0, rgb.1, rgb.2)
    }
}
