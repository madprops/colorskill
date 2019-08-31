use crate::*;

pub struct RGB
{
    red: u8,
    green: u8,
    blue: u8
}

impl RGB
{
    /// Makes a new RGB from three u8 values.
    /// 
    /// # Example
    /// 
    /// ```
    /// use colorskill::RGB;
    /// let c = RGB::new(34, 66, 94);
    /// ```
    pub fn new(red: u8, green: u8, blue: u8) -> RGB
    {
        RGB
        {
            red, green, blue
        }
    }

    /// Makes a new RGB from a u8 tuple.
    /// 
    /// # Example
    /// 
    /// ```
    /// use colorskill::RGB;
    /// let c = RGB::from_tuple((22, 95, 83));
    /// ```
    pub fn from_tuple(t: (u8, u8, u8)) -> RGB
    {
        RGB
        {
            red: t.0,
            green: t.1,
            blue: t.2
        }
    }

    /// Gets the red RGB value.
    /// 
    /// # Example
    /// 
    /// ```
    /// use colorskill::RGB;
    /// let c = RGB::new(34, 66, 94);
    /// let red = c.get_red();
    /// ```
    pub fn get_red(&self) -> u8
    {
        self.red
    }

    /// Gets the green RGB value.
    /// 
    /// # Example
    /// 
    /// ```
    /// use colorskill::RGB;
    /// let c = RGB::new(34, 66, 94);
    /// let green = c.get_green();
    /// ```
    pub fn get_green(&self) -> u8
    {
        self.green
    }

    /// Gets the blue RGB value.
    /// 
    /// # Example
    /// 
    /// ```
    /// use colorskill::RGB;
    /// let c = RGB::new(34, 66, 94);
    /// let blue = c.get_blue();
    /// ```
    pub fn get_blue(&self) -> u8
    {
        self.blue
    }

    /// Sets the red RGB value.
    /// 
    /// # Example
    /// 
    /// ```
    /// use colorskill::RGB;
    /// let mut c = RGB::new(34, 66, 94);
    /// c.set_red(80);
    /// ```
    pub fn set_red(&mut self, value: u8)
    {
        self.red = value;
    }

    /// Sets the green RGB value.
    /// 
    /// # Example
    /// 
    /// ```
    /// use colorskill::RGB;
    /// let mut c = RGB::new(34, 66, 94);
    /// c.set_green(80);
    /// ```
    pub fn set_green(&mut self, value: u8)
    {
        self.green = value;
    }

    /// Sets the blue RGB value.
    /// 
    /// # Example
    /// 
    /// ```
    /// use colorskill::RGB;
    /// let mut c = RGB::new(34, 66, 94);
    /// c.set_blue(80);
    /// ```
    pub fn set_blue(&mut self, value: u8)
    {
        self.blue = value;
    }

    /// Sets all the RGB values from a  tuple.
    /// 
    /// # Example
    /// 
    /// ```
    /// use colorskill::RGB;
    /// let mut c = RGB::new(34, 66, 94);
    /// c.set_from_tuple((19, 23, 129));
    /// ```
    pub fn set_from_tuple(&mut self, t: (u8, u8, u8))
    {
        self.set_red(t.0); self.set_green(t.1); self.set_blue(t.2);
    }

    /// Gets the RGB values in a tuple.
    /// 
    /// # Example
    /// 
    /// ```
    /// use colorskill::RGB;
    /// let c = RGB::new(34, 66, 94);
    /// let t = c.get_tuple();
    /// ```
    pub fn get_tuple(&self) -> (u8, u8, u8)
    {
        (self.get_red(), self.get_green(), self.get_blue())
    }

    /// Makes the RGB darker by an amount.
    /// 
    /// # Example
    /// 
    /// ```
    /// use colorskill::RGB;
    /// let mut c = RGB::new(34, 66, 94);
    /// c.make_darker(20.0);
    /// ```
    pub fn make_darker(&mut self, amount: f64)
    {
        self.set_from_tuple(make_color_darker(self.get_tuple(), amount));
    }

    /// Makes the RGB lighter by an amount.
    /// 
    /// # Example
    /// 
    /// ```
    /// use colorskill::RGB;
    /// let mut c = RGB::new(34, 66, 94);
    /// c.make_lighter(20.0);
    /// ```
    pub fn make_lighter(&mut self, amount: f64) 
    {
        self.set_from_tuple(make_color_lighter(self.get_tuple(), amount));
    }

    /// Randomizes the RGB values.
    /// 
    /// # Example
    /// 
    /// ```
    /// use colorskill::RGB;
    /// let mut c = RGB::new(34, 66, 94);
    /// c.randomize();
    /// ```
    pub fn randomize(&mut self)
    {
        self.set_from_tuple(random_color());
    }

    /// Returns the HSL hue value
    /// 
    /// # Example
    /// 
    /// ```
    /// use colorskill::RGB;
    /// let c = RGB::new(34, 66, 94);
    /// let hue = c.get_hue();
    /// ```
    pub fn get_hue(&self) -> f64
    {
        get_color_hue(self.get_tuple())
    }

    /// Returns the HSL saturation value
    /// 
    /// # Example
    /// 
    /// ```
    /// use colorskill::RGB;
    /// let c = RGB::new(34, 66, 94);
    /// let saturation = c.get_saturation();
    /// ```
    pub fn get_saturation(&self) -> f64
    {
        get_color_saturation(self.get_tuple())
    }

    /// Returns the HSL lightness value
    /// 
    /// # Example
    /// 
    /// ```
    /// use colorskill::RGB;
    /// let c = RGB::new(34, 66, 94);
    /// let lightness = c.get_lightness();
    /// ```
    pub fn get_lightness(&self) -> f64
    {
        get_color_lightness(self.get_tuple())
    }

    /// Turns the RGB into a string.
    /// 
    /// See the to_string definition 
    /// to check what the output is.
    /// 
    /// # Example
    /// 
    /// ```
    /// use colorskill::RGB;
    /// let c = RGB::new(34, 66, 94);
    /// let s = c.to_string();
    /// ```
    pub fn to_string(&self) -> String
    {
        color_to_string(self.get_tuple())
    }

    /// Turns the RGB into a string.
    /// 
    /// See the to_string_2 definition 
    /// to check what the output is.
    /// 
    /// # Example
    /// 
    /// ```
    /// use colorskill::RGB;
    /// let c = RGB::new(34, 66, 94);
    /// let s = c.to_string_2();
    /// ```
    pub fn to_string_2(&self) -> String
    {
        color_to_string_2(self.get_tuple())
    }

    /// Turns the RGB into a string.
    /// 
    /// See the to_string_3 definition 
    /// to check what the output is.
    /// 
    /// # Example
    /// 
    /// ```
    /// use colorskill::RGB;
    /// let c = RGB::new(34, 66, 94);
    /// let s = c.to_string_3("RGB");
    /// ```
    pub fn to_string_3(&self, prepend: &str) -> String
    {
        color_to_string_3(self.get_tuple(), prepend)
    }

    /// Turns the RGB into a string.
    /// 
    /// See the to_string_4 definition 
    /// to check what the output is.
    /// 
    /// # Example
    /// 
    /// ```
    /// use colorskill::RGB;
    /// let c = RGB::new(34, 66, 94);
    /// let s = c.to_string_4("RGB");
    /// ```
    pub fn to_string_4(&self, prepend: &str) -> String
    {
        color_to_string_4(self.get_tuple(), prepend)
    }

    /// Uses the parse function to 
    /// change the values of the RGB.
    /// 
    /// Check the parse_color definition
    /// to check how to use it.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use colorskill::RGB;
    /// let mut c = RGB::new(34, 66, 94);
    /// c.change("red");
    /// c.change("46,39,199");
    /// c.change("darker");
    /// c.change("lighter");
    /// c.change("random");
    /// ```
    pub fn change(&mut self, s: &str)
    {
        self.set_from_tuple(parse_color(s, self.get_tuple()));
    }
}

// Unit Tests

#[cfg(test)]
mod tests 
{
    use super::*;

    #[test]
    fn rgb_test()
    {
        let mut c = RGB::new(0, 0, 0);

        c.make_lighter(45.0);
        assert_eq!(c.get_red(), 115);
        assert_eq!(c.get_green(), 115);
        assert_eq!(c.get_blue(), 115);

        c.change("red");
        c.make_darker(45.0);
        assert_eq!(c.get_red(), 26);
        assert_eq!(c.get_green(), 0);
        assert_eq!(c.get_blue(), 0);

        c.set_red(100); 
        c.set_green(83); 
        c.set_blue(193);
        assert_eq!(c.get_red(), 100);
        assert_eq!(c.get_green(), 83);
        assert_eq!(c.get_blue(), 193);

        c.set_from_tuple((55, 129, 90));
        assert_eq!(c.get_red(), 55);
        assert_eq!(c.get_green(), 129);
        assert_eq!(c.get_blue(), 90);

        let t = c.get_tuple();
        assert_eq!(t.0, 55);
        assert_eq!(t.1, 129);
        assert_eq!(t.2, 90);

        assert_eq!(c.to_string(), "55,129,90");
        assert_eq!(c.to_string_2(), "55, 129, 90");
        assert_eq!(c.to_string_3(""), "(55,129,90)");
        assert_eq!(c.to_string_3("RGB"), "RGB(55,129,90)");
        assert_eq!(c.to_string_4(""), "(55, 129, 90)");
        assert_eq!(c.to_string_4("RGB"), "RGB(55, 129, 90)");

        let mut c2 = RGB::from_tuple((100, 90, 89));

        c2.change("lighter2");
        assert_eq!(c2.get_red(), 152);
        assert_eq!(c2.get_green(), 140);
        assert_eq!(c2.get_blue(), 139);

        assert_eq!(c2.get_hue(), 4.62);
        assert_eq!(c2.get_saturation(), 5.94);
        assert_eq!(c2.get_lightness(), 57.06);

        c2.randomize();
    }
}