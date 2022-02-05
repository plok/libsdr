pub struct Instrument {
    pub source_path: String,
    pub pattern: Vec<bool>,
    pub amplify: f32,
}

impl Instrument {
    #[inline]
    pub fn new(source_path: &str, pattern: Vec<u8>, amplify: Option<f32>) -> Instrument {
        Instrument {
            source_path: source_path.to_string(),
            pattern: pattern.iter().map(|p| p != &0).collect(),
            amplify: match amplify {
                Some(val) => val,
                _ => 1.0,
            },
        }
    }
}
