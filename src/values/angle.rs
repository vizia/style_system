use crate::{Calc, CustomParseError, DimensionPercentage, Parse, TryAdd};
use cssparser::*;

#[derive(Debug, Clone)]
pub enum Angle {
    Deg(f32),
    Grad(f32),
    Rad(f32),
    Turn(f32),
}

impl<'i> Parse<'i> for Angle {
    fn parse<'t>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        match input.try_parse(Calc::parse) {
            Ok(Calc::Value(v)) => return Ok(*v),
            Ok(_) => unreachable!(),
            _ => {}
        }

        let location = input.current_source_location();
        let token = input.next()?;
        match *token {
            Token::Dimension {
                value, ref unit, ..
            } => {
                match_ignore_ascii_case! { unit,
                    "deg" => Ok(Angle::Deg(value)),
                    "grad" => Ok(Angle::Grad(value)),
                    "turn" => Ok(Angle::Turn(value)),
                    "rad" => Ok(Angle::Rad(value)),
                    _ => return Err(location.new_unexpected_token_error(token.clone())),
                }
            }
            ref token => return Err(location.new_unexpected_token_error(token.clone())),
        }
    }
}

impl Angle {
    pub fn is_zero(&self) -> bool {
        use Angle::*;
        match self {
            Deg(v) | Rad(v) | Grad(v) | Turn(v) => *v == 0.0,
        }
    }

    pub fn to_radians(&self) -> f32 {
        const RAD_PER_DEG: f32 = std::f32::consts::PI / 180.0;
        match self {
            Angle::Deg(deg) => deg * RAD_PER_DEG,
            Angle::Rad(rad) => *rad,
            Angle::Grad(grad) => grad * 180.0 / 200.0 * RAD_PER_DEG,
            Angle::Turn(turn) => turn * 360.0 * RAD_PER_DEG,
        }
    }

    pub fn to_degrees(&self) -> f32 {
        const DEG_PER_RAD: f32 = 180.0 / std::f32::consts::PI;
        match self {
            Angle::Deg(deg) => *deg,
            Angle::Rad(rad) => rad * DEG_PER_RAD,
            Angle::Grad(grad) => grad * 180.0 / 200.0,
            Angle::Turn(turn) => turn * 360.0,
        }
    }
}

impl std::convert::Into<Calc<Angle>> for Angle {
    fn into(self) -> Calc<Angle> {
        Calc::Value(Box::new(self))
    }
}

impl std::convert::From<Calc<Angle>> for Angle {
    fn from(calc: Calc<Angle>) -> Angle {
        match calc {
            Calc::Value(v) => *v,
            _ => unreachable!(),
        }
    }
}

impl std::ops::Mul<f32> for Angle {
    type Output = Self;

    fn mul(self, other: f32) -> Angle {
        match self {
            Angle::Deg(v) => Angle::Deg(v * other),
            Angle::Rad(v) => Angle::Deg(v * other),
            Angle::Grad(v) => Angle::Deg(v * other),
            Angle::Turn(v) => Angle::Deg(v * other),
        }
    }
}

impl std::ops::Add<Angle> for Angle {
    type Output = Self;

    fn add(self, other: Angle) -> Angle {
        Angle::Deg(self.to_degrees() + other.to_degrees())
    }
}

impl TryAdd<Angle> for Angle {
    fn try_add(&self, other: &Angle) -> Option<Angle> {
        Some(Angle::Deg(self.to_degrees() + other.to_degrees()))
    }
}

impl std::cmp::PartialEq<f32> for Angle {
    fn eq(&self, other: &f32) -> bool {
        match self {
            Angle::Deg(a) | Angle::Rad(a) | Angle::Grad(a) | Angle::Turn(a) => a == other,
        }
    }
}

impl std::cmp::PartialEq<Angle> for Angle {
    fn eq(&self, other: &Angle) -> bool {
        self.to_degrees() == other.to_degrees()
    }
}

impl std::cmp::PartialOrd<f32> for Angle {
    fn partial_cmp(&self, other: &f32) -> Option<std::cmp::Ordering> {
        match self {
            Angle::Deg(a) | Angle::Rad(a) | Angle::Grad(a) | Angle::Turn(a) => a.partial_cmp(other),
        }
    }
}

impl std::cmp::PartialOrd<Angle> for Angle {
    fn partial_cmp(&self, other: &Angle) -> Option<std::cmp::Ordering> {
        self.to_degrees().partial_cmp(&other.to_degrees())
    }
}

// https://drafts.csswg.org/css-values-4/#typedef-angle-percentage
pub type AnglePercentage = DimensionPercentage<Angle>;

#[cfg(test)]
mod tests {

    use super::*;

    const VALID_ANGLE_DEG: &str = "30deg";

    #[test]
    fn parse_angle_deg() {
        let mut parser_input = ParserInput::new(&VALID_ANGLE_DEG);
        let mut parser = Parser::new(&mut parser_input);
        let result = Angle::parse(&mut parser);
        assert_eq!(result, Ok(Angle::Deg(30.0)));
    }

    const VALID_ANGLE_GRAD: &str = "30grad";

    #[test]
    fn parse_angle_grad() {
        let mut parser_input = ParserInput::new(&VALID_ANGLE_GRAD);
        let mut parser = Parser::new(&mut parser_input);
        let result = Angle::parse(&mut parser);
        assert_eq!(result, Ok(Angle::Grad(30.0)));
    }

    const VALID_ANGLE_RAD: &str = "30rad";

    #[test]
    fn parse_angle_rad() {
        let mut parser_input = ParserInput::new(&VALID_ANGLE_RAD);
        let mut parser = Parser::new(&mut parser_input);
        let result = Angle::parse(&mut parser);
        assert_eq!(result, Ok(Angle::Rad(30.0)));
    }

    const VALID_ANGLE_TURN: &str = "30turn";

    #[test]
    fn parse_angle_turn() {
        let mut parser_input = ParserInput::new(&VALID_ANGLE_TURN);
        let mut parser = Parser::new(&mut parser_input);
        let result = Angle::parse(&mut parser);
        assert_eq!(result, Ok(Angle::Turn(30.0)));
    }
}
