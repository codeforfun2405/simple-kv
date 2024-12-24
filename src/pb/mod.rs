pub mod kv;

use core::fmt;

use kv::{kv_command::Command, kv_command_resp::CommandResp, KeyVal, KvCommand, KvCommandResp};

impl KvCommand {
    pub fn new(cmd: Option<Command>) -> Self {
        KvCommand { command: cmd }
    }
}

impl Command {
    pub fn new_get(key: String) -> Self {
        Command::Get(kv::CmdGet { key })
    }

    pub fn new_set(key: String, value: Vec<u8>) -> Self {
        Command::Set(kv::CmdSet {
            key_val: Some(kv::KeyVal {
                key,
                value: value.into(),
            }),
        })
    }

    pub fn new_del(key: String) -> Self {
        Command::Del(kv::CmdDel { key })
    }
}

impl From<Command> for KvCommand {
    fn from(cmd: Command) -> Self {
        KvCommand::new(Some(cmd))
    }
}

impl KvCommandResp {
    pub fn new_get(status: String, key_val: Option<kv::KeyVal>) -> Self {
        KvCommandResp {
            command_resp: Some(CommandResp::GetResp(kv::CmdGetResp { status, key_val })),
        }
    }

    pub fn new_set(status: String, key_val: Option<kv::KeyVal>) -> Self {
        KvCommandResp {
            command_resp: Some(CommandResp::SetResp(kv::CmdSetResp { status, key_val })),
        }
    }

    pub fn new_del(status: String) -> Self {
        KvCommandResp {
            command_resp: Some(CommandResp::DelResp(kv::CmdDelResp { status })),
        }
    }
}

impl fmt::Display for KeyVal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "key: {}, value: {:?}", self.key, self.value)
    }
}
