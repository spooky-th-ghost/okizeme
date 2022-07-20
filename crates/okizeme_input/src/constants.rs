use regex::Regex;

use crate::{CommandMotion, CommandType};

lazy_static! {
    pub static ref MOTIONS: [CommandMotion; 8] = [
        CommandMotion::new(
            1,
            Regex::new("([^69]+[69]{1,5}[^6]{0,9}5[^5]{0,4}6)").unwrap(),
            CommandType::Dash
        ),
        CommandMotion::new(
            1,
            Regex::new("([^47]+[47]{1,5}[^4]{0,9}5[^5]{0,4}4)").unwrap(),
            CommandType::BackDash
        ),
        CommandMotion::new(
            2,
            Regex::new("(2[^2]{0,4}3[^3]{0,4}6)").unwrap(),
            CommandType::Qcf
        ),
        CommandMotion::new(
            2,
            Regex::new("(2[^2]{0,4}1[^1]{0,4}4)").unwrap(),
            CommandType::Qcb
        ),
        CommandMotion::new(
            3,
            Regex::new("(6[^6]{0,4}2[^2]{0,4}3)").unwrap(),
            CommandType::Dp
        ),
        CommandMotion::new(
            3,
            Regex::new("(4[^4]{0,4}2[^2]{0,4}1)").unwrap(),
            CommandType::BackDp
        ),
        CommandMotion::new(
            4,
            Regex::new("(6[^6]{0,6}2[^2]{0,6}4)").unwrap(),
            CommandType::Hcb
        ),
        CommandMotion::new(
            4,
            Regex::new("(4[^4]{0,6}2[^2]{0,6}6)").unwrap(),
            CommandType::Hcf
        ),
    ];
}
