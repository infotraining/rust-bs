use mockall::*;
use mockall::predicate::*;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Point(u32, u32);

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

#[derive(Debug, PartialEq)]
pub struct Board {
    _width: u32,
    _height: u32,
    _apples: Vec<Point>,
}

impl Board {
    pub fn new(width: u32, height: u32) -> Board {
        Board {
            _width: width,
            _height: height,
            _apples: Vec::new(),
        }
    }

    pub fn width(&self) -> u32 {
        return self._width;
    }

    pub fn height(&self) -> u32 {
        return self._height;
    }

    pub fn add_apple(&mut self, apple: Point) {
        self._apples.push(apple);
    }

    pub fn apples(&self) -> &Vec<Point> {
        &self._apples
    }

    pub fn try_to_eat_apple(&mut self, point: Point) -> bool {
        if let Some(index) = self._apples.iter().position(|&p| p == point) {
            self._apples.remove(index);
            return true;
        }
        false
    }
}

pub struct SnakeGame<'a> {
    board: Board,
    snake: Snake,
    terminal: &'a dyn Terminal
}

impl<'a> SnakeGame<'a> {
    // pub fn new(width: u32, height: u32) -> SnakeGame {    
    //     let game = SnakeGame {
    //         board : Board::new(width, height),
    //         snake : Snake::new(vec![(width / 2, height / 2), (width / 2, height / 2 + 1)])
    //     };

    //     game
    // }

    pub fn snake(&self) -> &Snake {
        &self.snake
    }

    pub fn run(&self) {
        self.terminal.render_board(&self.board);
        self.terminal.render_apples(self.board.apples());
    }
}

struct SnakeGameBuilder<'a> {
    board_dimensions: (u32, u32),
    apples: Vec<Point>,
    terminal: Option<&'a dyn Terminal>
}

impl<'a> SnakeGameBuilder<'a> {
    pub fn new() -> SnakeGameBuilder<'a> {
        SnakeGameBuilder {
            board_dimensions: (20, 10),
            apples: Vec::new(),
            terminal: None
        }
    }

    pub fn with_board(mut self, width: u32, height: u32) -> Self {
        self.board_dimensions = (width, height);
        self
    }

    pub fn with_apples(mut self, apples: Vec<Point>) -> Self {
        self.apples = apples;
        self
    }

    pub fn with_terminal(mut self, terminal: &'a (dyn Terminal + 'a)) -> Self {
        self.terminal = Some(terminal);
        self
    }

    pub fn build(self) -> SnakeGame<'a> {
        let mut board = Board::new(self.board_dimensions.0, self.board_dimensions.1);
        for apple in self.apples {
            board.add_apple(apple);
        }

        SnakeGame {
            board: board,
            snake: Snake::new(vec![(self.board_dimensions.0 / 2, self.board_dimensions.1 / 2), (self.board_dimensions.0 / 2, self.board_dimensions.1 / 2 + 1)]),
            terminal: self.terminal.unwrap()
        }
    }
}



#[cfg(test)]
mod snake_tests {
    use rstest::{fixture, rstest};

    use crate::{Board, Direction, Point, Snake};

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

#[cfg(test)]
mod board_tests {
    use crate::Board;

    #[test]
    fn board_has_width_and_height() {
        let board = Board::new(10, 20);

        assert_eq!(board.width(), 10);
        assert_eq!(board.height(), 20);
    }
}

#[automock]
trait Terminal {
    fn render_board(&self, board: &Board);
    fn render_apples(&self, apples: &[Point]);
}

#[cfg(test)]
mod snake_game_tests {
    use mockall::predicate;

    use crate::{Direction, MockTerminal, Point, Snake, SnakeGame, SnakeGameBuilder};
    

    #[test]
    fn when_game_starts_snake_is_in_the_middle_of_the_board()
    {
        let mock_terminal = MockTerminal::new();
        let (width, height) = (20, 10);

        let game = SnakeGameBuilder::new()
            .with_board(width, height)
            .with_terminal(&mock_terminal)
            .build();

        assert_eq!(game.snake(), &Snake::new(vec![(10, 5), (10, 6)]));

    }

    #[test]
    fn when_game_starts_snake_is_alive()
    {
        let mock_terminal = MockTerminal::new();
        let (width, height) = (20, 10);

        let game = SnakeGameBuilder::new()
            .with_board(width, height)
            .with_terminal(&mock_terminal)
            .build();

        assert_eq!(game.snake().is_alive(), true);

    }

    #[test]
    fn when_game_starts_snake_is_directed_up()
    {
        let mock_terminal = MockTerminal::new();
        let (width, height) = (20, 10);

        let game = SnakeGameBuilder::new()
            .with_board(width, height)
            .with_terminal(&mock_terminal)
            .build();

        assert_eq!(game.snake().direction(), Direction::Up);

    }

    #[test]
    fn game_loop_renders_board() {
        let mut mock_terminal = MockTerminal::new();
        mock_terminal.expect_render_board().times(1).return_const(());
        mock_terminal.expect_render_apples().times(1).return_const(());

        let (width, height) = (20, 10);
        let game = SnakeGameBuilder::new()
            .with_board(width, height)
            .with_terminal(&mut mock_terminal)
            .build();

        game.run();
    }

    #[test]
    fn game_loop_renders_apples() {
        let mut mock_terminal = MockTerminal::new();
        mock_terminal.expect_render_board().times(1).return_const(());
        mock_terminal.expect_render_apples()
            .with(predicate::eq(vec![Point(3, 2), Point(6, 4)])).times(1).return_const(());

        let (width, height) = (20, 10);
        let game = SnakeGameBuilder::new()
            .with_board(width, height)
            .with_apples(vec![Point(3, 2), Point(6, 4)])
            .with_terminal(&mut mock_terminal)
            .build();

        game.run();
    }
}

fn main() {
    println!("Hello, world!");
}
