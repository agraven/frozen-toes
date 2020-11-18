use iced::{Button, Column, Container, Element, Length, Sandbox, Settings, Text};

use crate::game::{Board, Field, BOARD_SIZE};

pub mod game;

fn main() {
    FrozenToes::run(Settings::default());
}

#[derive(Clone)]
struct FrozenToes {
    board: Board,
    player: Field,

    buttons: [iced::button::State; 9],
    reset_button: iced::button::State,
}

impl FrozenToes {
    pub fn new() -> Self {
        Self {
            board: Board::new(),
            player: Field::Cross,

            buttons: [iced::button::State::new(); 9],
            reset_button: iced::button::State::new(),
        }
    }
}

/// Message type.
#[derive(Debug, Clone, Copy)]
enum Message {
    Place { x: usize, y: usize },
    Restart,
}

impl Sandbox for FrozenToes {
    type Message = Message;

    fn new() -> Self {
        FrozenToes::new()
    }

    fn title(&self) -> String {
        String::from("Frozen Toes")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::Place { x, y } => {
                if self.board.get(x, y) == Field::Blank {
                    self.board.set(x, y, self.player);
                    self.player = self.player.flipped();
                }
            }
            Message::Restart => {
                self.board.reset();
            }
        }
    }

    fn view(&mut self) -> Element<Self::Message> {
        // Construct buttons. Seperated to prevent ownership issues with button states.
        let board = self.board.clone();
        let winner = self.board.winner();
        let mut buttons: Vec<Button<_>> = self
            .buttons
            .iter_mut()
            .enumerate()
            .map(|(i, state)| {
                let x = i % BOARD_SIZE;
                let y = i / BOARD_SIZE;

                let content = Container::new(Text::new(board.get(x, y).symbol().to_string()))
                    .width(100.into())
                    .height(100.into())
                    .center_x()
                    .center_y();
                let button = Button::new(state, content);

                // Only register click handler if winner is undecided.
                match winner {
                    Field::Blank => button.on_press(Message::Place { x, y }),
                    _ => button,
                }
            })
            .collect();

        // Build the grid and the rest of the interface
        let mut root = Column::new().push(Text::new(format!("It's {:?}'s turn", self.player)));
        for _ in 0..BOARD_SIZE {
            let mut row = iced::Row::new();
            for _ in 0..BOARD_SIZE {
                row = row.push(buttons.remove(0));
            }
            root = root.push(row);
        }
        root = root.push(
            Button::new(&mut self.reset_button, Text::new("Start a new game"))
                .on_press(Message::Restart),
        );
        if winner != Field::Blank {
            root = root.push(Text::new(format!("{:?} won!", winner)));
        }
        iced::Container::new(root)
            .height(Length::Fill)
            .width(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
