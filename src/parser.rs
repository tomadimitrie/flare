#[derive(Parser)]
#[grammar = "flare.pest"]
pub(crate) struct FlareParser;

#[cfg(test)]
mod tests {
    #![allow(non_fmt_panic)]

    use pest::{
        parses_to,
        consumes_to,
        fails_with
    };
    use crate::parser::{
        Rule,
        FlareParser,
    };

    #[test]
    fn parser_string_letters() {
        parses_to! {
            parser: FlareParser,
            input: "\"abc\"",
            rule: Rule::string,
            tokens: [
                string(0, 5)
            ]
        };
    }

    #[test]
    fn parser_string_numbers() {
        parses_to! {
            parser: FlareParser,
            input: "\"123\"",
            rule: Rule::string,
            tokens: [
                string(0, 5)
            ]
        };
    }

    #[test]
    fn parser_string_spaces() {
        parses_to! {
            parser: FlareParser,
            input: "\"  \"",
            rule: Rule::string,
            tokens: [
                string(0, 4)
            ]
        };
    }

    #[test]
    fn parser_string_mix() {
        parses_to! {
            parser: FlareParser,
            input: "\"  2  ab\"",
            rule: Rule::string,
            tokens: [
                string(0, 9)
            ]
        };
    }

    #[test]
    fn parser_string_empty() {
        parses_to! {
            parser: FlareParser,
            input: "\"\"",
            rule: Rule::string,
            tokens: [
                string(0, 2)
            ]
        };
    }

    #[test]
    fn parser_string_invalid_no_right_quote() {
        fails_with! {
            parser: FlareParser,
            input: "\"aaa",
            rule: Rule::string,
            positives: vec![Rule::string],
            negatives: vec![],
            pos: 0
        };
    }

    #[test]
    fn parser_string_invalid_no_left_quote() {
        fails_with! {
            parser: FlareParser,
            input: "aaa\"",
            rule: Rule::string,
            positives: vec![Rule::string],
            negatives: vec![],
            pos: 0
        };
    }

    #[test]
    fn parser_string_invalid_no_quotes() {
        fails_with! {
            parser: FlareParser,
            input: "aaa",
            rule: Rule::string,
            positives: vec![Rule::string],
            negatives: vec![],
            pos: 0
        };
    }

    #[test]
    fn parser_int() {
        parses_to! {
            parser: FlareParser,
            input: "123",
            rule: Rule::int,
            tokens: [
                int(0, 3)
            ]
        };
    }

    #[test]
    fn parser_int_with_delimiters() {
        parses_to! {
            parser: FlareParser,
            input: "1_2_3",
            rule: Rule::int,
            tokens: [
                int(0, 5)
            ]
        };
    }

    #[test]
    fn parser_int_with_letters() {
        fails_with! {
            parser: FlareParser,
            input: "a2a2",
            rule: Rule::int,
            positives: vec![Rule::int],
            negatives: vec![],
            pos: 0
        };
    }

    #[test]
    fn parser_true() {
        parses_to! {
            parser: FlareParser,
            input: "true",
            rule: Rule::bool,
            tokens: [
                bool(0, 4, [
                    trueSym(0, 4)
                ])
            ]
        };
    }

    #[test]
    fn parser_false() {
        parses_to! {
            parser: FlareParser,
            input: "false",
            rule: Rule::bool,
            tokens: [
                bool(0, 5, [
                    falseSym(0, 5)
                ])
            ]
        };
    }

    #[test]
    fn parser_float() {
        parses_to! {
            parser: FlareParser,
            input: "12.34",
            rule: Rule::float,
            tokens: [
                float(0, 5)
            ]
        };
    }

    #[test]
    fn parser_delimiters() {
        parses_to! {
            parser: FlareParser,
            input: "1_2.3_4",
            rule: Rule::float,
            tokens: [
                float(0, 7)
            ]
        };
    }

    #[test]
    fn parser_float_with_letters() {
        fails_with! {
            parser: FlareParser,
            input: "a12.34",
            rule: Rule::float,
            positives: vec![Rule::float],
            negatives: vec![],
            pos: 0
        };
    }

    #[test]
    fn parser_float_no_integer_part() {
        fails_with! {
            parser: FlareParser,
            input: ".34",
            rule: Rule::float,
            positives: vec![Rule::float],
            negatives: vec![],
            pos: 0
        };
    }

    #[test]
    fn parser_float_no_decimal_part() {
        fails_with! {
            parser: FlareParser,
            input: "12.",
            rule: Rule::float,
            positives: vec![Rule::float],
            negatives: vec![],
            pos: 0
        };
    }
}