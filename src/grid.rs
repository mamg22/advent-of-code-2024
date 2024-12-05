use std::slice::Iter;

pub struct Grid<T> {
    width: usize,
    height: usize,
    data: Vec<T>,
}

impl<T: Default + Clone> Grid<T> {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            data: vec![T::default(); width * height],
        }
    }

    pub fn from_text(text: &str, converter: fn(char) -> T) -> Self {
        let width = text.lines().next().unwrap().len();
        let height = text.lines().count();

        let data: Vec<T> = Vec::from_iter(text.chars().filter(|ch| *ch != '\n').map(converter));

        assert_eq!(width * height, data.len());

        Self {
            width,
            height,
            data,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn iter(&self) -> Iter<T> {
        self.data.iter()
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        if x > self.width - 1 || y > self.height - 1 {
            None
        } else {
            Some(&self.data[y * self.width + x])
        }
    }

    pub fn set(&mut self, x: usize, y: usize, value: T) -> Result<(), ()> {
        if x > self.width - 1 || y > self.height - 1 {
            Err(())
        } else {
            self.data[y * self.width + x] = value;
            Ok(())
        }
    }
}
