use bracket_lib::prelude::*;

enum GameMode {
    Menu,
    Playing,
    End,
}

struct State {
    mode: GameMode,
}

impl State {
    fn new() -> Self {
        State {
            mode: GameMode::Menu,
        }
    }

    fn restart(&mut self) {
        self.mode = GameMode::Playing;
    }

    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "~~ S N A K E ~~");
        ctx.print_centered(8, "Press [P] to play");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                _ => {}
            }
        }
    }

    fn dead(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "Snake is dead !!!");
        ctx.print_centered(8, "[P] Play again");
        ctx.print_centered(9, "[Q] Quit game");
        self.mode = GameMode::End;

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

    fn play(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "Press arrow keys to move");
        ctx.print_centered(8, "Press [ESC] to quit");
        self.mode = GameMode::Playing;

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::Escape => self.mode = GameMode::End,
                VirtualKeyCode::Up => ctx.print_centered(10, "Up"),
                VirtualKeyCode::Down => ctx.print_centered(10, "Down"),
                VirtualKeyCode::Left => ctx.print_centered(10, "Left"),
                VirtualKeyCode::Right => ctx.print_centered(10, "Right"),
                _ => {}
            }
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::Playing => self.play(ctx),
            GameMode::End => self.dead(ctx),
        }
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Snake game")
        .build()?;

    let game_state = State::new();

    main_loop(context, game_state)
}
