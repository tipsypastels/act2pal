use act2pal::*;

macro_rules! c {
    ($($byte:literal)*) => {
        Colors::from_act(&[$($byte),*])
    };
}

#[test]
fn test_act_1_color() {
    assert_eq!(
        c!(255 255 255 1 0 0).unwrap(),
        Colors::new(vec![(255, 255, 255).into()])
    );
}
