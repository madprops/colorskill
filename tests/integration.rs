use colorskill::*;

#[test]
fn integration_test()
{
    let c = (100, 110, 120);
    assert_eq!(make_color_darker(c, 20.0), (54, 59, 64));
    assert_eq!(make_color_lighter(c, 20.0), (152, 161, 170));

    let mut c2 = RGB::new(100, 110, 120);
    c2.change("darker2");
    assert_eq!(c2.get_tuple(), (54, 59, 64));
    assert_eq!(c2.get_hue(), 210.0);
    assert_eq!(c2.get_saturation(), 8.47);
    assert_eq!(c2.get_lightness(), 23.14);

    c2.randomize();
}