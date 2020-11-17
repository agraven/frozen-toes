use iced::{executor, Application, Command, Element, Settings, Text};

pub mod game;

fn main() {
    FrozenToes::run(Settings::default());
}

struct FrozenToes;

impl Application for FrozenToes {
    type Executor = executor::Default;
    type Message = ();
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        (FrozenToes, Command::none())
    }

    fn title(&self) -> String {
        String::from("Frozen Toes")
    }

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&mut self) -> Element<Self::Message> {
        Text::new("Beep boop").into()
    }
}
