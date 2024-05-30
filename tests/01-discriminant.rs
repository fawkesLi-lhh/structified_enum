use structified_enum::structify;

#[structify]
#[all_need_msg]
enum ImplicitDiscriminant {
    #[msg("11")]
    AA,
    #[msg("12")]
    B,
    #[msg("13")]
    C,
    #[msg("14")]
    D,
}

#[test]
fn test_implicit_discriminant() {
    assert_eq!(ImplicitDiscriminant::AA.value(), 0);
    assert_eq!(ImplicitDiscriminant::B.value(), 1);
    assert_eq!(ImplicitDiscriminant::C.value(), 2);
    assert_eq!(ImplicitDiscriminant::D.value(), 3);
}

#[structify]
enum ExplicitDiscriminant {
    A,
    B = 0b11,
    C,
    //    D = 1 << 10,
}

#[test]
fn test_explicit_discriminant() {
    assert_eq!(ExplicitDiscriminant::A.value(), 0);
    assert_eq!(ExplicitDiscriminant::B.value(), 0b11);
    assert_eq!(ExplicitDiscriminant::C.value(), 0b11 + 1);
    assert_eq!(ImplicitDiscriminant::C.msg(), "13");
    //   assert_eq!(ExplicitDiscriminant::D.value(), 1 << 10);
}

fn main() {}
