pub struct Raveai{
    distance:f64,
    break_distance:f64,
    deviation:f64,
}
impl Raveai {
    pub fn new() -> Raveai{
        Raveai{
            distance: 0.0,
            break_distance: 0.0,
            deviation: 0.0,
        }
    }
}