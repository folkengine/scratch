#[derive(Clone, Debug, PartialEq, Eq)]
struct Element {
    name: String,
    attributes: Vec<(String, String)>,
    children: Vec<Element>,
}

type ParseResult<'a, Output> = Result<(&'a str, Output), &'a str>;

trait Parser<'a, Output> {
    fn parse(&self, input: &'a str) -> ParseResult<'a, Output>;
}

impl<'a, F, Output> Parser<'a, Output> for F
where
    F: Fn(&'a str) -> ParseResult<Output>,
{
    fn parse(&self, input: &'a str) -> ParseResult<'a, Output> {
        self(input)
    }
}

mod pc {
    pub fn match_literal(expected: &'static str) -> impl Fn(&str) -> Result<(&str, ()), &str> {
        move |input| match input.get(0..expected.len()) {
            Some(next) if next == expected => Ok((&input[expected.len()..], ())),
            _ => Err(input),
        }
    }

    pub fn identifier(input: &str) -> Result<(&str, String), &str> {
        let mut matched = String::new();
        let mut chars = input.chars();

        match chars.next() {
            Some(next) if next.is_alphabetic() => matched.push(next),
            _ => return Err(input),
        }

        while let Some(next) = chars.next() {
            if next.is_alphanumeric() || next == '-' {
                matched.push(next);
            } else {
                break;
            }
        }

        let next_index = matched.len();
        Ok((&input[next_index..], matched))
    }

    pub fn pair<P1, P2, R1, R2>(
        parser1: P1,
        parser2: P2,
    ) -> impl Fn(&str) -> Result<(&str, (R1, R2)), &str>
    where
        P1: Fn(&str) -> Result<(&str, R1), &str>,
        P2: Fn(&str) -> Result<(&str, R2), &str>,
    {
        move |input| match parser1(input) {
            Ok((next_input, result1)) => match parser2(next_input) {
                Ok((final_input, result2)) => Ok((final_input, (result1, result2))),
                Err(err) => Err(err),
            },
            Err(err) => Err(err),
        }
    }

    pub fn map<P, F, A, B>(parser: P, map_fn: F) -> impl Fn(&str) -> Result<(&str, B), &str>
    where
        P: Fn(&str) -> Result<(&str, A), &str>,
        F: Fn(A) -> B,
    {
        move |input| parser(input).map(|(next_input, result)| (next_input, map_fn(result)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn match_literal() {
        let parse_joe = pc::match_literal("Hello Joe!");
        assert_eq!(Ok(("", ())), parse_joe("Hello Joe!"));
        assert_eq!(
            Ok((" Hello Robert!", ())),
            parse_joe("Hello Joe! Hello Robert!")
        );
        assert_eq!(Err("Hello Mike!"), parse_joe("Hello Mike!"));
    }

    #[test]
    fn identifier() {
        assert_eq!(
            Ok(("", "i-am-an-identifier".to_string())),
            pc::identifier("i-am-an-identifier")
        );
        assert_eq!(
            Ok((" entirely an identifier", "not".to_string())),
            pc::identifier("not entirely an identifier")
        );
        assert_eq!(
            Err("!not at all an identifier"),
            pc::identifier("!not at all an identifier")
        );
    }

    #[test]
    fn pair() {
        let tag_opener = pc::pair(pc::match_literal("<"), pc::identifier);
        assert_eq!(
            Ok(("/>", ((), "my-first-element".to_string()))),
            tag_opener("<my-first-element/>")
        );
        assert_eq!(Err("oops"), tag_opener("oops"));
        assert_eq!(Err("!oops"), tag_opener("<!oops"));
    }
}
