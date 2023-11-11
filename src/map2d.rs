pub struct Map2D {
    tiles: Vec<char>,
    width: usize,
    height: usize
}

/// Represents a contiguous set of tiles aligned in a 2D grid.
impl Map2D {
    ///
    /// Create a new map out of an input string. If the input string contains any empty lines, the remainder of the input will be ignored.
    /// 
    /// # Panics
    /// 
    /// Will panic if any line in the string has a different length than the first one.
    /// 
    /// # Examples
    /// ```
    /// let input = String::from("12345\n12345");
    /// let map = aoc_lib::map2d::Map2D::new(input);
    /// ```
    pub fn new(input: String) -> Map2D {
        let split: Vec<&str> = input.split("\n").take_while(|line| !line.is_empty()).collect();
        let width = split[0].len();
        for (idx, line) in split.iter().enumerate() {
            if line.len() != width {
                panic!("Invalid length on line {idx}. Expected {width}. Found {}. Full line: {line}", line.len());
            }
        }
        let height = split.len();
        Map2D {
            tiles: split.join("").chars().collect(),
            width,
            height
        }
    }

    /// Get the character at the given position.
    /// 
    /// # Examples
    /// ```
    /// let input = String::from("12345\n67890");
    /// let map = aoc_lib::map2d::Map2D::new(input);
    /// assert_eq!(Some('1'), map.get(0,0));
    /// assert_eq!(Some('7'), map.get(1,1));
    /// assert_eq!(None, map.get(0,2));
    /// ```
    pub fn get(&self, x: usize, y: usize) -> Option<char> {
        let idx = self.get_index(x, y);
        match self.is_in_bounds(x, y) {
            true => Some(self.tiles[idx]),
            false => None
        }
    }

    /// Set a position to the given value.
    /// 
    /// # Examples
    /// ```
    /// let input = String::from("12345\n67890");
    /// let mut map = aoc_lib::map2d::Map2D::new(input);
    /// map.set(4, 1, 'A');
    /// assert_eq!(Some('A'), map.get(4,1));
    /// ```
    pub fn set(&mut self, x: usize, y: usize, val: char) {
        let idx = self.get_index(x, y);
        self.tiles[idx] = val;
    }

    fn get_index(&self, x: usize, y: usize) -> usize {
        x + y * self.width
    }

    /// Check if the given coordinates are in bounds.
    /// 
    /// # Examples
    /// ```
    /// let input = String::from("12345\n12345\n12345");
    /// let map = aoc_lib::map2d::Map2D::new(input);
    /// assert_eq!(map.is_in_bounds(5, 2), false);
    /// assert_eq!(map.is_in_bounds(4, 3), false);
    /// assert_eq!(map.is_in_bounds(4, 2), true);
    /// assert_eq!(map.is_in_bounds(0, 0), true);
    /// ```
    pub fn is_in_bounds(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }

    /// Prints the map to the console.
    pub fn print(&self) {
        for (idx, chara) in self.tiles.iter().enumerate() {
            if idx % self.width == 0 {
                println!();
            }
            print!("{chara}");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_new_panics() {
        let input = String::from("123456\n12345");
        Map2D::new(input);
    }

    #[test]
    fn test_new() {
        let input = String::from("12345\n12345\n12345");
        let map = Map2D::new(input);
        assert_eq!(map.width, 5);
        assert_eq!(map.height, 3);

        let input = String::from("12345\n12345\n12345\n");
        let map = Map2D::new(input);
        assert_eq!(map.width, 5);
        assert_eq!(map.height, 3);

        let input = String::from("12345\n12345\n12345\n\n12345");
        let map = Map2D::new(input);
        assert_eq!(map.width, 5);
        assert_eq!(map.height, 3);
    }

    #[test]
    fn test_print() {
        let input = String::from("12345\n12345\n12345");
        let map = Map2D::new(input);
        map.print();
    }
}