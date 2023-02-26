pub fn round(weight: f32, granularity: f32) -> f32 {
    (weight * granularity / 10.).round() as f32 * 10. / granularity
}
