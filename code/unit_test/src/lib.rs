fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("cannot divide by zero"))
    } else {
        Ok(a / b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn division_returns_ok() -> Result<(), String> {
        assert_eq!(divide(10, 2)?, 5);
        Ok(())
    }

    #[test]
    fn division_returns_err() {
        assert_eq!(divide(10, 0), Err(String::from("cannot divide by zero")));
    }
}