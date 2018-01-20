//! Example library for great CI integration!

use std::fmt;

/// Greetings to some target
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Greeting<'a>(&'a str);

impl<'a> Greeting<'a> {
    /// Construct a Greeter to greet a target
    pub fn greet(target: &str) -> Greeting {
        Greeting(target)
    }
}

impl<'a> fmt::Display for Greeting<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Greetings, {}!", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn greet_author() {
        let greeter = Greeting::greet("CAD97");
        assert_eq!(greeter.to_string(), "Greetings, CAD97!");
    }
}
