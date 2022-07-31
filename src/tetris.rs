use std::collections::HashSet;

use crate::shape::{Position, Shape};

#[derive(Debug)]
pub struct Tetris {
    width: i32,
    height: i32,
    current_shape: Shape,
    fixed_shapes: Vec<Shape>,
    lost: bool,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Direction {
    Left,
    Right,
}

impl Tetris {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width: width as i32,
            height: height as i32,
            current_shape: &Shape::new_random() + Position(((width as i32) - 1) / 2, 0),
            fixed_shapes: vec![],
            lost: false,
        }
    }

    pub fn out_of_bounds(&self, shape: &Shape) -> bool {
        !shape
            .iter_positions()
            .all(|pos| pos.0 >= 0 && pos.0 < self.width && pos.1 >= 0 && pos.1 < self.height)
    }

    pub fn colliding(&self, shape: &Shape) -> bool {
        self.fixed_shapes
            .iter()
            .any(|fixed_shape| fixed_shape.collides_with(shape))
    }

    pub fn tick(&mut self) {
        if self.lost {
            return;
        }
        let translated_current_shape = &self.current_shape + Position(0, 1);
        if self.out_of_bounds(&translated_current_shape)
            || self.colliding(&translated_current_shape)
        {
            let new_fixed_shape = std::mem::replace(
                &mut self.current_shape,
                &Shape::new_random() + Position((self.width - 1) / 2, 0),
            );
            self.fixed_shapes.push(new_fixed_shape);
            self.remove_full_lines();
            if self.colliding(&self.current_shape) {
                self.lost = true;
            }
        } else {
            self.current_shape = translated_current_shape;
        }
    }

    pub fn shift(&mut self, direction: Direction) {
        if self.lost {
            return;
        }
        let translated_current_shape = &self.current_shape
            + match direction {
                Direction::Left => Position(-1, 0),
                Direction::Right => Position(1, 0),
            };
        if !self.out_of_bounds(&translated_current_shape)
            && !self.colliding(&translated_current_shape)
        {
            self.current_shape = translated_current_shape;
        }
    }

    pub fn rotate(&mut self) {
        if self.lost {
            return;
        }
        let rotated_current_shape = self.current_shape.rotated();
        if !self.out_of_bounds(&rotated_current_shape) && !self.colliding(&rotated_current_shape) {
            self.current_shape = rotated_current_shape;
        }
    }

    pub fn line_full(&self, y: i32) -> bool {
        self.fixed_shapes
            .iter()
            .flat_map(|shape| shape.iter_positions())
            .filter(|position| position.1 == y)
            .collect::<HashSet<_>>()
            .len() as i32
            == self.width
    }

    pub fn iter_positions(&self) -> impl Iterator<Item = Position> {
        let height = self.height;
        let width = self.width;
        (0..height).flat_map(move |y| (0..width).map(move |x| Position(x, y)))
    }

    pub fn get(&self, position: Position) -> Option<&'static str> {
        if self.current_shape.has_position(&position) {
            return Some(self.current_shape.typ());
        }
        self.fixed_shapes
            .iter()
            .find(|shape| shape.has_position(&position))
            .map(|shape| shape.typ())
    }

    fn remove_line(&mut self, y: i32) {
        for shape in self.fixed_shapes.iter_mut() {
            shape.remove_line(y);
        }
    }

    fn remove_full_lines(&mut self) {
        for y in 0..self.height {
            if self.line_full(y) {
                self.remove_line(y);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Tetris;

    #[test]
    fn tests() {
        let mut tetris = Tetris::new(10, 30);
        tetris.tick();
        tetris.tick();
        tetris.tick();
        println!("{:#?}", tetris);
    }
}
