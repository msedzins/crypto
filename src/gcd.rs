pub fn calculate<T>(x: T, y: T) -> T
where
    T: Default + std::cmp::PartialEq + std::ops::Rem<Output = T> + Copy
{
    let reminder = x % y;
    if reminder == T::default() {
        return y;
    }
    return calculate(y, reminder);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_gcd_u32() {
        
        let x: u32  = 2024;
        let y: u32 = 748;
        let x: u32 = calculate(x, y);
        
        assert_eq!(x, 44);
    }

    #[test]
    fn calculate_gcd_i32() {
        
        let x: i32  = 2024;
        let y: i32 = 748;
        let x: i32 = calculate(x, y);
        
        assert_eq!(x, 44);
    }

    #[test]
    fn calculate_gcd_i8() {
        
        let x: i8  = 120;
        let y: i8 = 14;
        let x: i8 = calculate(x, y);
        
        assert_eq!(x, 2);
    }
}
