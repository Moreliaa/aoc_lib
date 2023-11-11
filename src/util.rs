pub fn manhattan(x1: usize, y1: usize, x2: usize, y2: usize) -> usize {
    x1.abs_diff(x2) + y1.abs_diff(y2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manhattan() {
        assert_eq!(manhattan(0, 0, 0, 0), 0);
        assert_eq!(manhattan(1, 10, 100, 1000), 99 + 990);
    }

}