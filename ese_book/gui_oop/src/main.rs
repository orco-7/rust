use gui::Draw;

pub struct selectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for selectBox {
    fn draw(&self) {
        todo!()
    }
}


use gui::{Button, Screen};

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(selectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("No"),
                    String::from("Maybe"),
                ],
            }),
            Box::new(Button {
                width: 5,
                height: 10,
                label: String::from("Ok"),
            }),
        ]
    };

    screen.run();
}
