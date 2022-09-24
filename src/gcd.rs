pub fn calculate(x: u32, y: u32) -> u32 {
    let reminder = x % y;
    if reminder == 0 {
        return y;
    }
    return calculate(y, reminder);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_gcd() {
        let x = calculate(2024, 748);
        assert_eq!(x, 44);
    }
}
