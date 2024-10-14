#[derive(Debug, PartialEq, Clone)]
struct Segment(u32, u32);

impl Segment {
    fn x(&self) -> u32 {
        self.0
    } 

    fn y(&self) -> u32 {
        self.1
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Snake {
    pub segments: Vec<Segment>,
    is_alive_: bool
}

pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Snake {
    pub fn new(segments: Vec<(u32, u32)>) -> Snake {
        let mut snake = Snake{segments: Vec::new(), is_alive_: false };
        for s in segments {
            snake.segments.push(Segment(s.0, s.1));
        }
        snake
    }

    pub fn head(&self) -> &Segment {
        &self.segments[0]
    }

    pub fn tail(&self) -> &Segment {
        &self.segments[self.segments.len() - 1]
    }

    pub fn crawl(&mut self, direction: Direction) {
        let Segment(x, y) = *self.head();

        let new_head = match direction {
            Direction::Left => Segment(x - 1, y),
            Direction::Right => Segment(x + 1, y),
            Direction::Up => Segment(x, y - 1),
            Direction::Down => Segment(x, y + 1),
        };

        self.segments.insert(0, new_head.clone());
        self.segments.pop();

        if new_head.x() == 0 || new_head.y() == 0 {
            self.is_alive_ = false;
        }
    }

    pub fn is_alive(&self) -> bool {
        self.is_alive_
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

    use crate::{Direction, Snake, Board, Segment};

    #[test]
    fn snake_constructed_with_segments() {
        let snake = Snake::new(vec![(0, 0), (1, 0), (2, 0)]);

        assert_eq!(snake.head(), &Segment(0, 0));
        assert_eq!(snake.tail(), &Segment(2, 0));
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

    #[rstest]
    #[case(Snake::new(vec![(10, 1)]), Direction::Up)]
    #[case(Snake::new(vec![(10, 9)]), Direction::Down)]
    #[case(Snake::new(vec![(1, 5)]), Direction::Left)]
    #[case(Snake::new(vec![(9, 5)]), Direction::Right)]
    fn snake_dies_when_hits_the_wall(#[case] mut snake: Snake, #[case] direction: Direction) {
        let board = Board(20, 10);

        snake.crawl(direction);

        assert!(!snake.is_alive());
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
