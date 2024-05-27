pub fn factorial(num: u64) -> u64 {
    if num == 0{
        return 1;
    }
    return num * factorial(num - 1);
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = factorial(5);
        assert_eq!(result, 120);
    }
}
