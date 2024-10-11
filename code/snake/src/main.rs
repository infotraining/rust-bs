type Segment = (i32, i32);

#[derive(Debug, Clone, PartialEq)]
pub struct Snake {
    pub segments: Vec<Segment>,
}

pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Snake {
    pub fn new(segments: Vec<Segment>) -> Snake {
        Snake { segments }
    }

    pub fn head(&self) -> &Segment {
        &self.segments[0]
    }

    pub fn tail(&self) -> &Segment {
        &self.segments[self.segments.len() - 1]
    }

    pub fn crawl(&mut self, direction: Direction) {
        let (x, y) = *self.head();

        let new_head = match direction {
            Direction::Left => (x - 1, y),
            Direction::Right => (x + 1, y),
            Direction::Up => (x, y - 1),
            Direction::Down => (x, y + 1),
        };

        self.segments.insert(0, new_head);
        self.segments.pop();
    }
}

#[derive(Debug, PartialEq)]
pub struct Board(u32, u32);

impl Board {
    pub fn width(&self) -> u32 {
        return self.0;
    }

    pub fn height(&self) -> u32 {
        return self.1;
    }
}

#[cfg(test)]
mod snake_tests {
    use rstest::rstest;

    use crate::{Direction, Snake};

    #[test]
    fn snake_constructed_with_segments() {
        let snake = Snake::new(vec![(0, 0), (1, 0), (2, 0)]);

        assert_eq!(snake.head(), &(0, 0));
        assert_eq!(snake.tail(), &(2, 0));
    }

    #[rstest]
    #[case(Snake::new(vec![(5, 5), (5, 6), (5, 7)]), Direction::Right, Snake::new(vec![(6, 5), (5, 5), (5, 6)]))]
    #[case(Snake::new(vec![(5, 5), (5, 6), (5, 7)]), Direction::Left, Snake::new(vec![(4, 5), (5, 5), (5, 6)]))]
    #[case(Snake::new(vec![(5, 5), (5, 6), (5, 7)]), Direction::Up, Snake::new(vec![(5, 4), (5, 5), (5, 6)]))]
    #[case(Snake::new(vec![(5, 7), (5, 6), (5, 5)]), Direction::Down, Snake::new(vec![(5, 8), (5, 7), (5, 6)]))]
    fn snake_moves_in_given_direction(
        #[case] mut snake: Snake,
        #[case] direction: Direction,
        #[case] expected_snake: Snake,
    ) {
        snake.crawl(direction);
        assert_eq!(snake, expected_snake);
    }
}

#[cfg(test)]
mod board_tests {
    use crate::Board;

    #[test]
    fn board_has_width_and_height() {
        let board = Board(10, 20);

        assert_eq!(board.width(), 10);
        assert_eq!(board.height(), 20);
    }
}

fn main() {
    println!("Hello, world!");
}
