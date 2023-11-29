/// Calculate the manhattan distance between two points in 2D space.
///
/// # Examples
/// ```
/// assert_eq!(aoc_lib::util::manhattan_2d((0, 0), (0, 0)), 0);
/// assert_eq!(aoc_lib::util::manhattan_2d((1, 10), (100, 1000)), 99 + 990);
/// assert_eq!(aoc_lib::util::manhattan_2d((-1, -10), (100, 1000)), 101 + 1010);
/// ```
pub fn manhattan_2d(a: (i32, i32), b: (i32, i32)) -> i32 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

/// Calculate the manhattan distance between two points in 3D space.
///
/// # Examples
/// ```
/// assert_eq!(aoc_lib::util::manhattan_3d((0, 0, 0), (0, 0, 0)), 0);
/// assert_eq!(aoc_lib::util::manhattan_3d((1, 10, 3), (100, 1000, 5)), 99 + 990 + 2);
/// assert_eq!(aoc_lib::util::manhattan_3d((-1, -10, -8), (100, 1000, -12)), 101 + 1010 + 4);
/// ```
pub fn manhattan_3d(a: (i32, i32, i32), b: (i32, i32, i32)) -> i32 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs() + (a.2 - b.2).abs()
}
