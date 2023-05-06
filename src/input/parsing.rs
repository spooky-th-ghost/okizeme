use crate::input::types::{ButtonMask, CommandMotion};

pub type ParseResult<'a, Output> = Result<(&'a str, Output), &'a str>;

pub trait Parser<'a, Output> {
    fn parse(&self, input: &'a str) -> ParseResult<'a, Output>;

    fn map<F, NewOutput>(self, map_fn: F) -> BoxedParser<'a, NewOutput>
    where
        Self: Sized + 'a,
        Output: 'a,
        NewOutput: 'a,
        F: Fn(Output) -> NewOutput + 'a,
    {
        BoxedParser::new(map(self, map_fn))
    }

    fn pred<F>(self, pred_fn: F) -> BoxedParser<'a, Output>
    where
        Self: Sized + 'a,
        Output: 'a,
        F: Fn(&Output) -> bool + 'a,
    {
        BoxedParser::new(pred(self, pred_fn))
    }

    fn and_then<F, NextParser, NewOutput>(self, f: F) -> BoxedParser<'a, NewOutput>
    where
        Self: Sized + 'a,
        Output: 'a,
        NewOutput: 'a,
        NextParser: Parser<'a, NewOutput> + 'a,
        F: Fn(Output) -> NextParser + 'a,
    {
        BoxedParser::new(and_then(self, f))
    }
}

impl<'a, F, Output> Parser<'a, Output> for F
where
    F: Fn(&'a str) -> ParseResult<Output>,
{
    fn parse(&self, input: &'a str) -> ParseResult<'a, Output> {
        self(input)
    }
}

pub struct BoxedParser<'a, Output> {
    parser: Box<dyn Parser<'a, Output> + 'a>,
}

impl<'a, Output> BoxedParser<'a, Output> {
    fn new<P>(parser: P) -> Self
    where
        P: Parser<'a, Output> + 'a,
    {
        BoxedParser {
            parser: Box::new(parser),
        }
    }
}

impl<'a, Output> Parser<'a, Output> for BoxedParser<'a, Output> {
    fn parse(&self, input: &'a str) -> ParseResult<'a, Output> {
        self.parser.parse(input)
    }
}
fn map<'a, P, F, A, B>(parser: P, map_fn: F) -> impl Parser<'a, B>
where
    P: Parser<'a, A>,
    F: Fn(A) -> B,
{
    move |input| {
        parser
            .parse(input)
            .map(|(next_input, result)| (next_input, map_fn(result)))
    }
}

fn pred<'a, P, A, F>(parser: P, predicate: F) -> impl Parser<'a, A>
where
    P: Parser<'a, A>,
    F: Fn(&A) -> bool,
{
    move |input| {
        if let Ok((next_input, value)) = parser.parse(input) {
            if predicate(&value) {
                return Ok((next_input, value));
            }
        }
        Err(input)
    }
}

fn and_then<'a, P, F, A, B, NextP>(parser: P, f: F) -> impl Parser<'a, B>
where
    P: Parser<'a, A>,
    NextP: Parser<'a, B>,
    F: Fn(A) -> NextP,
{
    move |input| match parser.parse(input) {
        Ok((next_input, result)) => f(result).parse(next_input),
        Err(err) => Err(err),
    }
}
fn either<'a, P1, P2, A>(parser1: P1, parser2: P2) -> impl Parser<'a, A>
where
    P1: Parser<'a, A>,
    P2: Parser<'a, A>,
{
    move |input| match parser1.parse(input) {
        ok @ Ok(_) => ok,
        Err(_) => parser2.parse(input),
    }
}

fn pair<'a, P1, P2, R1, R2>(parser1: P1, parser2: P2) -> impl Parser<'a, (R1, R2)>
where
    P1: Parser<'a, R1>,
    P2: Parser<'a, R2>,
{
    move |input| {
        parser1.parse(input).and_then(|(next_input, result1)| {
            parser2
                .parse(next_input)
                .map(|(last_input, result2)| (last_input, (result1, result2)))
        })
    }
}

pub fn one_or_more<'a, P, A>(parser: P) -> impl Parser<'a, Vec<A>>
where
    P: Parser<'a, A>,
{
    move |mut input| {
        let mut result = Vec::new();

        if let Ok((next_input, first_item)) = parser.parse(input) {
            input = next_input;
            result.push(first_item);
        } else {
            return Err(input);
        }

        while let Ok((next_input, next_item)) = parser.parse(input) {
            input = next_input;
            result.push(next_item);
        }

        Ok((input, result))
    }
}

fn repeat_x_times<'a, P, A>(parser: P, upper_bound: usize) -> impl Parser<'a, Vec<A>>
where
    P: Parser<'a, A>,
{
    move |mut input| {
        let mut result = Vec::new();

        if let Ok((next_input, first_item)) = parser.parse(input) {
            input = next_input;
            result.push(first_item);
        } else {
            return Err(input);
        }

        'primary: while let Ok((next_input, next_item)) = parser.parse(input) {
            input = next_input;
            result.push(next_item);
            if result.len() == upper_bound {
                break 'primary;
            }
        }

        Ok((input, result))
    }
}

fn zero_or_more<'a, P, A>(parser: P) -> impl Parser<'a, Vec<A>>
where
    P: Parser<'a, A>,
{
    move |mut input| {
        let mut result = Vec::new();

        while let Ok((next_input, next_item)) = parser.parse(input) {
            input = next_input;
            result.push(next_item);
        }

        Ok((input, result))
    }
}

fn any_char(input: &str) -> ParseResult<char> {
    match input.chars().next() {
        Some(next) => Ok((&input[next.len_utf8()..], next)),
        _ => Err(input),
    }
}

fn match_literal<'a>(expected: &'static str) -> impl Parser<'a, ()> {
    move |input: &'a str| match input.get(0..expected.len()) {
        Some(next) if next == expected => Ok((&input[expected.len()..], ())),
        _ => Err(input),
    }
}

pub mod motion_parsing {
    use super::*;

    pub fn one_or_more<'a, P, A>(parser: P) -> impl Parser<'a, Vec<A>>
    where
        P: Parser<'a, A>,
    {
        move |mut input| {
            let mut result = Vec::new();

            if let Ok((next_input, first_item)) = parser.parse(input) {
                input = next_input;
                result.push(first_item);
            } else {
                return Err(input);
            }

            while let Ok((next_input, next_item)) = parser.parse(input) {
                input = next_input;
                result.push(next_item);
            }

            Ok((input, result))
        }
    }
    // pub fn capture_all_motions(input: &str, buttons: Vec<ButtonMask>) -> InputTree {
    //     let mut motions = Vec::new();
    //     let dqcf = double_qcf();
    //     let dp = one_or_more(dp());
    //     let rdp = one_or_more(rdp());
    //     let qcf = one_or_more(qcf());
    //     let qcb = one_or_more(qcb());
    //     let two_two = one_or_more(two_two());
    //     let dash = one_or_more(dash());
    //     let backdash = one_or_more(backdash());

    //     match dqcf.parse(input) {
    //         Ok((remaining, motion)) => {
    //             motions.push((motion, input.len() - remaining.len()));
    //         }
    //         _ => (),
    //     }
    //     match dp.parse(input) {
    //         Ok((remaining, mut found_motions)) => {
    //             if let Some(motion) = found_motions.pop() {
    //                 motions.push((motion, input.len() - remaining.len()));
    //             }
    //         }
    //         _ => (),
    //     }
    //     match rdp.parse(input) {
    //         Ok((remaining, mut found_motions)) => {
    //             if let Some(motion) = found_motions.pop() {
    //                 motions.push((motion, input.len() - remaining.len()));
    //             }
    //         }
    //         _ => (),
    //     }
    //     match qcf.parse(input) {
    //         Ok((remaining, mut found_motions)) => {
    //             if let Some(motion) = found_motions.pop() {
    //                 motions.push((motion, input.len() - remaining.len()));
    //             }
    //         }
    //         _ => (),
    //     }
    //     match qcb.parse(input) {
    //         Ok((remaining, mut found_motions)) => {
    //             if let Some(motion) = found_motions.pop() {
    //                 motions.push((motion, input.len() - remaining.len()));
    //             }
    //         }
    //         _ => (),
    //     }
    //     match two_two.parse(input) {
    //         Ok((remaining, mut found_motions)) => {
    //             if let Some(motion) = found_motions.pop() {
    //                 motions.push((motion, input.len() - remaining.len()));
    //             }
    //         }
    //         _ => (),
    //     }
    //     match dash.parse(input) {
    //         Ok((remaining, mut found_motions)) => {
    //             if let Some(motion) = found_motions.pop() {
    //                 motions.push((motion, input.len() - remaining.len()));
    //             }
    //         }
    //         _ => (),
    //     }
    //     match backdash.parse(input) {
    //         Ok((remaining, mut found_motions)) => {
    //             if let Some(motion) = found_motions.pop() {
    //                 motions.push((motion, input.len() - remaining.len()));
    //             }
    //         }
    //         _ => (),
    //     }

    //     InputTree {
    //         motions,
    //         last_direction: 5,
    //     }
    // }

    pub fn double_qcf<'a>() -> impl Parser<'a, CommandMotion> {
        pair(
            qcf(),
            pair(
                pair(
                    repeat_x_times(any_char.pred(|c| *c != '2'), 5),
                    repeat_x_times(match_literal("2"), 8),
                ),
                pair(repeat_x_times(match_literal("3"), 5), match_literal("6")),
            ),
        )
        .map(|(_, _)| CommandMotion::DoubleQcf)
    }

    pub fn dash<'a>() -> impl Parser<'a, CommandMotion> {
        pair(
            pair(
                zero_or_more(any_char.pred(|c| *c != '6')),
                repeat_x_times(match_literal("6"), 10),
            ),
            pair(repeat_x_times(match_literal("5"), 6), match_literal("6")),
        )
        .map(|(_next_input, _result)| CommandMotion::Dash)
    }

    pub fn backdash<'a>() -> impl Parser<'a, CommandMotion> {
        pair(
            pair(
                zero_or_more(any_char.pred(|c| *c != '4')),
                repeat_x_times(match_literal("4"), 10),
            ),
            pair(repeat_x_times(match_literal("5"), 6), match_literal("4")),
        )
        .map(|(_next_input, _result)| CommandMotion::Backdash)
    }

    pub fn qcf<'a>() -> impl Parser<'a, CommandMotion> {
        pair(
            pair(
                zero_or_more(any_char.pred(|c| *c != '2')),
                repeat_x_times(match_literal("2"), 8),
            ),
            pair(repeat_x_times(match_literal("3"), 5), match_literal("6")),
        )
        .map(|(_next_input, _result)| CommandMotion::Qcf)
    }

    pub fn qcb<'a>() -> impl Parser<'a, CommandMotion> {
        pair(
            pair(
                zero_or_more(any_char.pred(|c| *c != '2')),
                repeat_x_times(match_literal("2"), 8),
            ),
            pair(repeat_x_times(match_literal("1"), 5), match_literal("4")),
        )
        .map(|(_next_input, _result)| CommandMotion::Qcb)
    }

    pub fn dp<'a>() -> impl Parser<'a, CommandMotion> {
        pair(
            pair(
                zero_or_more(any_char.pred(|c| *c != '6')),
                repeat_x_times(match_literal("6"), 8),
            ),
            pair(repeat_x_times(match_literal("2"), 5), match_literal("3")),
        )
        .map(|(_next_input, _result)| CommandMotion::Dp)
    }

    pub fn rdp<'a>() -> impl Parser<'a, CommandMotion> {
        pair(
            pair(
                zero_or_more(any_char.pred(|c| *c != '4')),
                repeat_x_times(match_literal("4"), 8),
            ),
            pair(repeat_x_times(match_literal("2"), 5), match_literal("1")),
        )
        .map(|(_next_input, _result)| CommandMotion::Rdp)
    }

    pub fn two_two<'a>() -> impl Parser<'a, CommandMotion> {
        pair(
            pair(
                zero_or_more(any_char.pred(|c| *c != '2')),
                repeat_x_times(match_literal("2"), 10),
            ),
            pair(repeat_x_times(match_literal("5"), 6), match_literal("2")),
        )
        .map(|(_next_input, _result)| CommandMotion::TwoTwo)
    }

    pub fn find_command_motion<'a>() -> impl Parser<'a, CommandMotion> {
        either(
            double_qcf(),
            either(
                dp(),
                either(
                    rdp(),
                    either(
                        qcf(),
                        either(qcb(), either(two_two(), either(dash(), backdash()))),
                    ),
                ),
            ),
        )
        .map(|result| result)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use motion_parsing::*;

    #[test]
    fn all_inputs_test() {
        let find_it = find_command_motion();

        assert_eq!(
            find_it.parse("554422233654"),
            Ok(("54", CommandMotion::Qcf))
        )
    }

    #[test]
    fn double_qcf_test() {
        let find_it = double_qcf();
        assert_eq!(
            find_it.parse("55522233366552223336"),
            Ok(("", CommandMotion::DoubleQcf))
        );

        assert_eq!(find_it.parse("555222333665555552223336"), Err("552223336"));
    }

    #[test]
    fn dash_test() {
        let find_it = dash();
        assert_eq!(find_it.parse("556666655556"), Ok(("", CommandMotion::Dash)));
        assert_eq!(find_it.parse("555555555551111136"), Err(""));
    }

    #[test]
    fn backdash_test() {
        let find_it = backdash();
        assert_eq!(
            find_it.parse("55444455554"),
            Ok(("", CommandMotion::Backdash))
        );
        assert_eq!(find_it.parse("555555555551111136"), Err(""));
    }

    #[test]
    fn qcf_test() {
        let find_it = qcf();
        assert_eq!(
            find_it.parse("555555555552222233336"),
            Ok(("", CommandMotion::Qcf))
        );
        assert_eq!(find_it.parse("555555555551111136"), Err(""));
    }

    #[test]
    fn qcb_test() {
        let find_it = qcb();
        assert_eq!(
            find_it.parse("555555555552222211114"),
            Ok(("", CommandMotion::Qcb))
        );
        assert_eq!(find_it.parse("55555555511111133336"), Err(""));
    }

    #[test]
    fn dp_test() {
        let find_it = dp();
        assert_eq!(
            find_it.parse("55555555555666622223"),
            Ok(("", CommandMotion::Dp))
        );
        assert_eq!(find_it.parse("55555555511111133336"), Err(""));
    }

    #[test]
    fn rdp_test() {
        let find_it = rdp();
        assert_eq!(
            find_it.parse("55555666644422221"),
            Ok(("", CommandMotion::Rdp))
        );
        assert_eq!(find_it.parse("55555555511111133336"), Err(""));
    }

    #[test]
    fn two_two_test() {
        let find_it = two_two();
        assert_eq!(find_it.parse("5522255552"), Ok(("", CommandMotion::TwoTwo)));
        assert_eq!(find_it.parse("555555555551111136"), Err(""));
    }
}
