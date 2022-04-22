#[cfg(test)]
mod tests {
    use lib_rgb::Colour;
    use lib_rgb::Gradient;

    #[test]
    fn it_works() {

        // let c = lib_rgb::Colour{r:2,g:4,b:255};
        // let result = 2 + 2;
        // assert_eq!(result, 4);
    }

    fn rainbow_test(position: f32, expected_colour: Colour) {
        let rainbow = lib_rgb::UnicornVomit {};
        let red = rainbow.get(position);
        assert_eq!(red, expected_colour);
    }

    #[test]
    fn rainbow_red() {
        rainbow_test(0.0, Colour::RED);
        rainbow_test(1.0, Colour::RED);
    }

    #[test]
    fn rainbow_yellow() {
        rainbow_test(1.0 / 6.0, Colour::YELLOW);
    }
    
    #[test]
    fn rainbow_green() {
        rainbow_test(2.0 / 6.0, Colour::GREEN);
    }

    #[test]
    fn rainbow_cyan() {
        rainbow_test(3.0 / 6.0, Colour::CYAN);
    }

    #[test]
    fn rainbow_blue() {
        rainbow_test(4.0 / 6.0, Colour::BLUE);
    }

    #[test]
    fn rainbow_magenta() {
        rainbow_test(5.0 / 6.0, Colour::MAGENTA);
    }
}
