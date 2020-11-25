use std::str::FromStr;
use std::fmt;

#[derive(PartialEq, Debug, Clone)]
pub enum Cmd {
    None,
    Use,
    Where,
    Send,
    Cd,
    Rec,
    Restart,
    Others,
    Finish,
    Start,

}

impl Cmd {
    pub fn from_str(input: &str) -> Self {
        match input {
            "cd" => Cmd::Cd,
            "send" => Cmd::Send,
            "rec" => Cmd::Rec,
            "use" => Cmd::Use,
            "where" => Cmd::Where,
            "restart" => Cmd::Restart,
            "finish" => Cmd::Finish,
            "start"=>Cmd::Send,
            "" => Cmd::None,
            _=>Cmd::Others,
        }
    }
    pub fn to_string(&self) -> String {
        let res = {
            match self {
                Cmd::Send => "send",
                Cmd::Cd => "cd",
                Cmd::Rec => "rec",
                Cmd::Where => "where",
                Cmd::Use => "use",
                Cmd::Restart => "restart",
                Cmd::None=> "none",
                Cmd::Others => "others",
                Cmd::Finish=> "finish",
                Cmd::Start=> "start",
            }
        };
        res.to_string()
    }
}


