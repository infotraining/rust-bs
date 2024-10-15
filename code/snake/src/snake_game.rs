use mockall::*;
use mockall::predicate::*;

use crate::board::Board;
use crate::snake::{Point, Snake, Direction};

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


#[automock]
trait Terminal {
    fn render_board(&self, board: &Board);
    fn render_apples(&self, apples: &[Point]);
}

#[cfg(test)]
mod snake_game_tests {
    use mockall::predicate;
    use crate::snake_game::{MockTerminal, SnakeGameBuilder};

    use crate::snake::{Direction, Point, Snake};
    

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


