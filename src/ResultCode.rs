
#[derive(Copy, Clone, Debug,PartialEq, Eq)]
pub enum ResultCode{
    NOERROR = 0,
    FORMERR = 1,
    SERVFAIL = 2,
    NXDOMAIN = 3,
    NOTIMP = 4,
    REFUSE = 5,
}


impl ResultCode {
    pub fn from_num(num:u8) -> ResultCode{
        match num {
            1 => ResultCode::FORMERR,
            2 => ResultCode::SERVFAIL,
            3 => ResultCode::NXDOMAIN,
            4 => ResultCode::NOTIMP,
            5 => ResultCode::REFUSE,
            0 | _  => ResultCode::NOERROR
        }
    }
}