use crate::CommandMotion;

pub struct ActionMap {
    pub dash: bool,
    pub backdash: bool,
    pub qcf: Option<String>,
    pub qcb: Option<String>,
    pub dp: Option<String>,
    pub rdp: Option<String>,
    pub two_two: Option<String>,
    pub double_qcf: Option<String>,
}

impl ActionMap {
    pub fn contains(&self, current_motion: CommandMotion) -> Option<String> {
        use CommandMotion::*;
        match current_motion {
            Dash if self.dash => Some("".to_string()),
            Backdash if self.backdash => Some("".to_string()),
            Qcf if self.qcf.is_some() => self.qcf.clone(),
            Qcb if self.qcb.is_some() => self.qcb.clone(),
            Dp if self.dp.is_some() => self.dp.clone(),
            Rdp if self.rdp.is_some() => self.rdp.clone(),
            TwoTwo if self.two_two.is_some() => self.two_two.clone(),
            DoubleQcf if self.double_qcf.is_some() => self.double_qcf.clone(),
            _ => None,
        }
    }
}
