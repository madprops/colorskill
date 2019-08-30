- Get a color by name
- Make a color lighter
- Make a color darker
- Get a random color
- Parse color strings

---

```
pub fn color_name_to_rgb(name: &str, fallback: (u8, u8, u8)) -> (u8, u8, u8)
```
```
Gets an RGB tuple from a color name
The input is lowercased and the whitespaces are removed
So "Light Blue" will match "lightblue"
A fallback RGB tuple must be provided
```
```
color_name_to_rgb("firebrick", (0, 0, 0))
```

---

```
pub fn check_color_name(name: &str) -> bool
```
```
Checks if a color name exists
```
```
check_color_name("silver")
```

---

```
pub fn color_to_string(c: (u8, u8, u8)) -> String
```
```
Converts a color tuple
into a comma separated string
(0, 0, 0) -> "0,0,0"
```
```
color_to_string((100, 143, 49))
```

---

```
pub fn color_to_string_2(c: (u8, u8, u8)) -> String
```
```
Converts a color tuple
into a comma separated string
with spaces after commas
(0, 0, 0) -> "0, 0, 0"
```
```
color_to_string_2((100, 143, 49))
```

---

```
pub fn color_to_string_3(c: (u8, u8, u8)) -> String
```
```
Converts a color tuple
into a comma separated string
with added parenthesis
(0, 0, 0) -> "(0,0,0)"
```
```
color_to_string_3((100, 143, 49))
```

---

```
pub fn color_to_string_4(c: (u8, u8, u8)) -> String
```
```
Converts a color tuple
into a comma separated string
with added parenthesis
and spaces after commas
(0, 0, 0) -> "(0, 0, 0)"
```
```
color_to_string_4((100, 143, 49))
```

---

```
pub fn random_color() -> (u8, u8, u8)
```
```
Generates a random RGB tuple
```
```
random_color()
```

---

```
pub fn parse_color(ans: &str, reference: (u8, u8, u8)) -> (u8, u8, u8)
```
```
Parses a color string
Useful for interpreting user input
Valid inputs can be:
"red", "0,0,0", "0, 0, 0"
"darker", "darker2", "darker3"
"lighter", "lighter2", "lighter3"
or "random" to get a random color
The input is lowercased and the whitespaces are removed
darker3 turns it 3 times darker than darker
Degrees for darker and lighter are hardcoded:
DEGREES_1: f64 = 15.0
DEGREES_2: f64 = 30.0
DEGREES_3: f64 = 45.0
```
```
parse_color("blue", (0, 0, 0))
parse_color("34,65,39", (0, 0, 0))
parse_color("darker", (10, 34, 50))
parse_color("random", (0, 0, 0))
```

---

```
fn make_color_darker(t: (u8, u8, u8), amount: f64) -> (u8, u8, u8)
```
```
Wrapper function to make a color darker
Receives a tuple and the amount to make darker
```
```
make_color_darker((43, 56, 84), 20.0)
```

---

```
fn make_color_lighter(t: (u8, u8, u8), amount: f64) -> (u8, u8, u8)
```
```
Wrapper function to make a color lighter
Receives a tuple and the amount to make lighter
```
```
make_color_lighter((43, 56, 84), 20.0)
```

---

```
pub fn change_color_lightness(t: (u8, u8, u8), darker: bool, amount: f64) -> (u8, u8, u8)
```
```
Turns a color darker or lighter
The amount represents HSL lightness degrees
Lightness goes from 0 to 359 degrees
The bigger the amount, the more it gets
darker or lighter
```
```
change_color_lightness((43, 56, 84), true, 20.0)
```