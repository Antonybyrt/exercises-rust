pub trait Power {
    fn power(self, n: u32) -> u32;
}

impl Power for u32 {
    fn power(self, n: u32) -> u32 {
        self.pow(n)
    }
}

impl Power for u16 {
    fn power(self, n: u32) -> u32 {
        (self as u32).pow(n)
    }
}

#[cfg(test)]
mod tests {
    use super::Power;

    #[test]
    fn test_power_u16() {
        let x: u32 = 2_u32.power(3u16 as u32);
        assert_eq!(x, 8);
    }

    #[test]
    fn test_power_u32() {
        let x: u32 = 2_u32.power(3u32);
        assert_eq!(x, 8);
    }

    #[test]
    fn test_power_ref_u32() {
        let x: u32 = 2_u32.power(3u32);
        assert_eq!(x, 8);
    }
}
