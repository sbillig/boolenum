use boolenum::BoolEnum;

#[derive(BoolEnum, Copy, Clone, Debug, PartialEq)]
pub enum TrueFalse {
    True,
    False,
}

fn main() {
    let yes: bool = TrueFalse::True.into();
    let no: bool = TrueFalse::False.into();
    assert!(yes);
    assert!(!no);
    assert_eq!(TrueFalse::from(true), TrueFalse::True);
    assert_eq!(TrueFalse::from(false), TrueFalse::False);
    assert_eq!(!TrueFalse::True, TrueFalse::False);
    assert_eq!(!TrueFalse::False, TrueFalse::True);
}
