use rand::Rng;

// Lowercase and remove whitespace
pub fn clean_string(s: &str) -> String
{
    s.to_lowercase().replace(" ", "")
}

// Converts a tuple to a colorsys Hsl
pub fn get_hsl(t: (u8, u8, u8)) -> colorsys::Hsl
{
    let rgb = colorsys::Rgb::from
    (
        (
            f64::from(t.0),
            f64::from(t.1),
            f64::from(t.2)
        )
    );

    colorsys::Hsl::from(&rgb)
}

// Returns an RGB tuple from an Hsl
pub fn get_rgb_tuple_from_hsl(hsl: &colorsys::Hsl) -> (u8, u8, u8)
{
    // Convert to RGB
    let rgb = colorsys::Rgb::from(hsl);

    // Return RGB tuple
    (
        rgb.get_red().round() as u8, 
        rgb.get_green().round() as u8, 
        rgb.get_blue().round() as u8
    )
}

// Rounds a float to 2 decimal numbers
pub fn round_float(n: f64) -> f64
{
    (n * 100.0).round() / 100.0
}

// Gets a random u8 number
pub fn random_u8() -> u8
{
    let mut rng = rand::thread_rng();
    let n: u8 = rng.gen(); n
}

#[cfg(test)]
mod tests 
{
    use super::*;

    #[test]
    fn utils_test()
    {
        assert_eq!(clean_string("A  momenT   Lapse"), "amomentlapse");
        assert_eq!(round_float(40.842135), 40.84);
        random_u8();
    }
}