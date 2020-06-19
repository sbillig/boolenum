use boolenum::BoolEnum;

#[derive(BoolEnum, Copy, Clone, Debug, PartialEq)]
pub enum WithValues {
    No = 0,
    Yes = 1,
}

fn main() {}
