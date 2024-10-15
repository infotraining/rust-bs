use crate::board::Board;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Point(pub u32, pub u32);

impl Point {
    pub fn x(&self) -> u32 {
        self.0
    }

    pub fn y(&self) -> u32 {
        self.1
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug)]
pub struct Snake {
    pub segments: Vec<Point>,
    _direction: Direction,
    _is_alive: bool,
}

impl Snake {
    pub fn new(segments: Vec<(u32, u32)>) -> Snake {
        let mut snake = Snake {
            segments: Vec::new(),
            _direction: Direction::Up,
            _is_alive: true,
        };
        for s in segments {
            snake.segments.push(Point(s.0, s.1));
        }
        snake
    }

    pub fn head(&self) -> &Point {
        &self.segments[0]
    }

    pub fn move_to(&mut self, direction: Direction, board: &mut Board) {
        self._direction = direction;
        
        let Point(x, y) = *self.head();

        let new_head = match direction {
            Direction::Left => Point(x - 1, y),
            Direction::Right => Point(x + 1, y),
            Direction::Up => Point(x, y - 1),
            Direction::Down => Point(x, y + 1),
        };


        self.segments.insert(0, new_head.clone());

        if new_head.x() == board.width()
            || new_head.y() == board.height()
            || new_head.x() == 0
            || new_head.y() == 0
        {
            self._is_alive = false;
        }

        if board.try_to_eat_apple(new_head) {
            return;
        }

        self.segments.pop();
    }

    pub fn is_alive(&self) -> bool {
        self._is_alive
    }

    pub fn direction(&self) -> Direction {
        self._direction
    }
}

impl PartialEq for Snake {
    fn eq(&self, other: &Self) -> bool {
        self.segments == other.segments
    }
}

#[cfg(test)]
mod snake_tests {
    use crate::board::Board;
    use crate::snake::{Snake, Direction, Point};
    use rstest::{fixture, rstest};

    #[test]
    fn snake_constructed_with_segments() {
        let snake = Snake::new(vec![(0, 0), (1, 0), (2, 0)]);

        assert_eq!(snake.head(), &Point(0, 0));
    }

    #[fixture]
    fn board() -> Board {
        Board::new(20, 10)
    }

    #[rstest]
    #[case(Snake::new(vec![(5, 5), (5, 6), (5, 7)]), Direction::Right, Snake::new(vec![(6, 5), (5, 5), (5, 6)]))]
    #[case(Snake::new(vec![(5, 5), (5, 6), (5, 7)]), Direction::Left, Snake::new(vec![(4, 5), (5, 5), (5, 6)]))]
    #[case(Snake::new(vec![(5, 5), (5, 6), (5, 7)]), Direction::Up, Snake::new(vec![(5, 4), (5, 5), (5, 6)]))]
    #[case(Snake::new(vec![(5, 7), (5, 6), (5, 5)]), Direction::Down, Snake::new(vec![(5, 8), (5, 7), (5, 6)]))]
    fn snake_moves_in_given_direction(
        mut board: Board,
        #[case] mut snake: Snake,
        #[case] direction: Direction,
        #[case] expected_snake: Snake,
    ) {
        snake.move_to(direction, &mut board);
        assert_eq!(snake, expected_snake);
        assert_eq!(snake.direction(), direction);
    }

    #[rstest]
    #[case(vec![(10, 1)], Direction::Up)]
    #[case(vec![(10, 9)], Direction::Down)]
    #[case(vec![(1, 5)], Direction::Left)]
    #[case(vec![(19, 5)], Direction::Right)]
    fn snake_dies_when_hits_the_wall(
        mut board: Board,
        #[case] segments: Vec<(u32, u32)>,
        #[case] direction: Direction,
    ) {
        let mut snake = Snake::new(segments);

        assert!(snake.is_alive());

        snake.move_to(direction, &mut board);

        assert!(!snake.is_alive());
    }

    #[rstest]
    fn when_snake_eats_apple_it_grows(mut board: Board) {
        board.add_apple(Point(5, 4));

        let mut snake = Snake::new(vec![(5, 5), (5, 6), (5, 7)]);

        snake.move_to(Direction::Up, &mut board);

        assert_eq!(snake, Snake::new(vec![(5, 4), (5, 5), (5, 6), (5, 7)]));
        assert_eq!(board.apples().len(), 0);
    }
}