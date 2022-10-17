use serde_derive::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct Cmd {
    command: String,
    opt_args: Vec<(String, String)>,
    args: Vec<String>,
}

impl Cmd {
    fn new(command: String, opt_args: Vec<(String, String)>, args: Vec<String>) -> Self {
        Self {
            command,
            opt_args,
            args,
        }
    }

    fn new_from_str(str: &str) -> Self {
        let cmd: Cmd = serde_json::from_str(str).unwrap();
        cmd
    }

    fn new_from_bytes(b_cmd: &[u8]) -> Self {
        let str = std::str::from_utf8(b_cmd).unwrap();
        Self::new_from_str(str)
    }

    fn to_string(&self) -> String {
        serde_json::to_string(&self).expect("could not serialize")
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.to_string().into_bytes()
    }
}

#[derive(Default)]
struct CmdParser {
    cmd: Cmd,
    hostnames: Vec<String>,
}

impl CmdParser {
    fn new() -> Self {
        Self::default()
    }

    fn parse_rm(str: &str) -> Self {
        let str_split: Vec<&str> = str.split(" ").collect();
        let str_command: &str = str_split.get(0).unwrap().as_ref();
        let command = String::from(str_command);

        let str_hostnames: &str = str_split.get(1).unwrap().as_ref();
        let hostnames: Vec<String> = str_hostnames
            .split(",")
            .map(|hostname| String::from(hostname))
            .collect();

        let str_args: &str = str_split.get(2).unwrap().as_ref();
        let args: Vec<String> = str_args
            .split(",")
            .map(|hostname| String::from(hostname))
            .collect();

        let opt_args = Vec::<(String, String)>::default();
        let cmd = Cmd::new(command, opt_args, args);
        Self { cmd, hostnames }
    }
}
