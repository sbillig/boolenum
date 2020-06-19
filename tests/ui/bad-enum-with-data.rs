use boolenum::BoolEnum;

#[derive(BoolEnum)]
pub enum EnumWithData {
    Yes(String),
    No,
}

fn main() {}
