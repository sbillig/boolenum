use boolenum::BoolEnum;

#[derive(BoolEnum, Copy, Clone, Debug, PartialEq)]
pub enum Good {
    No,
    Yes,
}

fn main() {
    let yes: bool = Good::Yes.into();
    let no: bool = Good::No.into();
    assert!(yes);
    assert!(!no);
    assert_eq!(Good::from(true), Good::Yes);
    assert_eq!(Good::from(false), Good::No);
    assert_eq!(!Good::Yes, Good::No);
    assert_eq!(!Good::No, Good::Yes);
}
