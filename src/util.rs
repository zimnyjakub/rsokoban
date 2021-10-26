
pub fn clamp<T: PartialOrd>(val: T, min: T, max: T) -> T {
    match val {
        val if val < min => min,
        val if val > max => max,
        val => val
    }
}

mod test {
    use super::*;

    #[test]
    fn test_clamp_min() {
        assert_eq!(clamp(-1, 0 , 10), 0);
    }
    #[test]
    fn test_clamp_max() {
        assert_eq!(clamp(11, 0 , 10), 10);
    }
    #[test]
    fn test_clamp_within() {
        assert_eq!(clamp(5, 0 , 10), 5);
    }
}