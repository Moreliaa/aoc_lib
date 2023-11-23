use std::fmt::Display;
use std::ops::Add;

/// Represents a contiguous set of tiles aligned in a 2D grid.
pub struct Map2D<T> {
    tiles: Vec<T>,
    width: i32,
    height: i32
}

impl<T> Map2D<T> {
    /// Create a new map with all tiles initialized with the given value.
    /// 
    /// # Examples
    /// ```
    /// let map = aoc_lib::map2d::Map2D::<char>::new(5, 4, 'A');
    /// assert_eq!(map.get(4,3), Some(&'A'));
    /// ```
    pub fn new(width: i32, height: i32, initial_value: T) -> Map2D<T> 
    where T: Clone
    {
        Map2D {
            tiles: vec![initial_value; (width * height) as usize],
            width,
            height
        }
    }

    /// Get the character at the given position.
    /// 
    /// # Examples
    /// ```
    /// let input = String::from("12345\n67890");
    /// let map = aoc_lib::map2d::Map2D::from_string(input);
    /// assert_eq!(Some(&'1'), map.get(0,0));
    /// assert_eq!(Some(&'7'), map.get(1,1));
    /// assert_eq!(None, map.get(0,2));
    /// ```
    pub fn get(&self, x: i32, y: i32) -> Option<&T> {
        match self.is_in_bounds(x, y) {
            true => Some(&self.tiles[self.get_index(x as i32, y as i32)]),
            false => None
        }
    }

    /// Set a position to the given value.
    /// 
    /// # Examples
    /// ```
    /// let input = String::from("12345\n67890");
    /// let mut map = aoc_lib::map2d::Map2D::from_string(input);
    /// map.set(4, 1, 'A');
    /// assert_eq!(Some(&'A'), map.get(4,1));
    /// ```
    pub fn set(&mut self, x: i32, y: i32, val: T) {
        if !self.is_in_bounds(x, y) {
            return;
        }
        let idx = self.get_index(x as i32, y as i32);
        self.tiles[idx] = val;
    }

    fn get_index(&self, x: i32, y: i32) -> usize {
        (x + y * self.width) as usize
    }

    /// Check if the given coordinates are in bounds.
    /// 
    /// # Examples
    /// ```
    /// let input = String::from("12345\n12345\n12345");
    /// let map = aoc_lib::map2d::Map2D::from_string(input);
    /// assert_eq!(map.is_in_bounds(5, 2), false);
    /// assert_eq!(map.is_in_bounds(4, 3), false);
    /// assert_eq!(map.is_in_bounds(4, 2), true);
    /// assert_eq!(map.is_in_bounds(0, 0), true);
    /// ```
    pub fn is_in_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && y >= 0 && (x as i32) < self.width && (y as i32) < self.height
    }

    /// Prints the map to the console.
    pub fn print(&self) 
    where T: Display
    {
        for (idx, chara) in self.tiles.iter().enumerate() {
            if idx % self.width as usize == 0 {
                println!();
            }
            print!("{}", chara);
        }
    }

    /// Aggregates values in the map into a single value.
    /// 
    /// # Arguments
    /// 
    /// `f` - a closure returning the value that should be aggregated.
    /// 
    /// # Returns
    /// The aggregated value.
    /// 
    /// # Examples
    /// ```
    /// let map = aoc_lib::map2d::Map2D::<i32>::new(10, 10, 1);
    /// assert_eq!(map.aggregate(|val| *val), 100);
    /// 
    /// let map = aoc_lib::map2d::Map2D::<Vec<i32>>::new(1, 10, vec![1,2]);
    /// assert_eq!(map.aggregate(|val| val[0] + val[1]), 30);
    /// ```
    pub fn aggregate<F, R>(&self, f: F) -> R 
    where
        R: Add<Output = R>,
        F: FnOnce(&T) -> R + Copy
    {
        let mut value: Option<R> = None;
        for tile in &self.tiles {
            match value {
                Some(val) => value = Some(val + f(tile)),
                None => value = Some(f(tile))
            }
        }
        value.unwrap()
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }
}

impl Map2D<char> {
    /// Create a new map out of an input string. If the input string contains any empty lines, the remainder of the input will be ignored.
    /// 
    /// # Panics
    /// 
    /// Will panic if any line in the string has a different length than the first one.
    /// 
    /// # Examples
    /// ```
    /// let input = String::from("12345\n12345");
    /// let map = aoc_lib::map2d::Map2D::from_string(input);
    /// ```
    pub fn from_string(input: String) -> Map2D<char> {
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
            width: width as i32,
            height: height as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_from_string_panics() {
        let input = String::from("123456\n12345");
        Map2D::from_string(input);
    }

    #[test]
    fn test_from_string() {
        let input = String::from("12345\n12345\n12345");
        let map = Map2D::from_string(input);
        assert_eq!(map.width, 5);
        assert_eq!(map.height, 3);

        let input = String::from("12345\n12345\n12345\n");
        let map = Map2D::from_string(input);
        assert_eq!(map.width, 5);
        assert_eq!(map.height, 3);

        let input = String::from("12345\n12345\n12345\n\n12345");
        let map = Map2D::from_string(input);
        assert_eq!(map.width, 5);
        assert_eq!(map.height, 3);
    }

    #[test]
    fn test_print() {
        let input = String::from("12345\n12345\n12345");
        let map = Map2D::from_string(input);
        map.print();
    }
}