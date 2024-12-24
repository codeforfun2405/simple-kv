// get data from tcp stream
// parse the data into cmd
// apply cmd to the storage

pub mod client_codec;
pub mod codec;

use crate::{
    pb::kv::{
        kv_command::{self, Command},
        KeyVal, KvCommand, KvCommandResp,
    },
    store::KvStore,
};
use anyhow::Result;
use codec::KvCommandCodec;
use futures::SinkExt;
use tokio::net::TcpStream;
use tokio_stream::StreamExt;
use tokio_util::codec::Framed;

pub async fn stream_handle(stream: TcpStream, kv_store: &mut KvStore) -> anyhow::Result<()> {
    let mut framed = Framed::new(stream, KvCommandCodec);

    loop {
        match framed.next().await {
            Some(Ok(cmd)) => {
                if cmd.command.is_none() {
                    return Ok(());
                }

                println!("received cmd: {:?}", cmd);
                let cmd_resp = handle_cmd(cmd, kv_store).await?;

                println!("send resp: {:?}", cmd_resp);
                framed.send(cmd_resp).await?
            }
            Some(Err(e)) => return Err(e),
            None => return Ok(()),
        }
    }
}

async fn handle_cmd(cmd: KvCommand, store: &mut KvStore) -> Result<KvCommandResp, anyhow::Error> {
    match cmd.command {
        Some(c) => match c {
            Command::Get(get) => {
                let key = get.key;
                let value = store.get(key.clone());

                let status = if value.is_some() { "ok" } else { "not found" };
                let mut key_val = None;

                match status {
                    "ok" => {
                        key_val = Some(KeyVal {
                            key: key.clone(),
                            value: value.unwrap(),
                        })
                    }
                    "not found" => {}
                    _ => {}
                }

                Ok(KvCommandResp::new_get(status.to_string(), key_val))
            }
            kv_command::Command::Set(set) => {
                let kv = set.key_val.unwrap();
                store.set(kv.clone());

                Ok(KvCommandResp::new_set("ok".to_string(), Some(kv)))
            }
            kv_command::Command::Del(del) => {
                store.del(del.key);
                Ok(KvCommandResp::new_del("ok".to_string()))
            }
        },
        None => Ok(KvCommandResp { command_resp: None }),
    }
}
