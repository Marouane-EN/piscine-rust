pub fn km_per_hour_to_meters_per_second(km_h: f64) -> f64 {
    (km_h * (10.0_f64).powf(3.0)) / 3600.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let km_h = 100.0;
        let m_s = km_per_hour_to_meters_per_second(km_h);
        assert_eq!(m_s, 27.77777777777778);
    }
}
