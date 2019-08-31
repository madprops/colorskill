use colorskill::*;

#[test]
fn integration_test()
{
    let c = (100, 100, 100);
    assert_eq!(make_color_darker(c, 20.0), (49, 49, 49));
    assert_eq!(make_color_lighter(c, 20.0), (151, 151, 151));

    let mut c2 = RGB::new(100, 100, 100);
    c2.change("darker2");
    assert_eq!(c2.get_tuple(), (49, 49, 49));
    assert_eq!(c2.get_hue(), 0.0);
    assert_eq!(c2.get_saturation(), 0.0);
    assert_eq!(c2.get_lightness(), 19.22);

    c2.randomize();
}