// get data from tcp stream
// parse the data into cmd
// apply cmd to the storage

use crate::{
    pb::kv::{
        kv_command,
        kv_command_resp::{self},
        CmdDelResp, CmdGetResp, CmdSetResp, KeyVal, KvCommand, KvCommandResp,
    },
    store::KvStore,
};
use anyhow::Result;
use bytes::BytesMut;
use futures::SinkExt;
use prost::Message;
use tokio::net::TcpStream;
use tokio_stream::StreamExt;
use tokio_util::codec::{Decoder, Encoder, Framed};

// parse the data into cmd
#[derive(Debug)]
pub struct KvCommandCodec;

pub async fn stream_handle(stream: TcpStream, kv_store: &mut KvStore) -> anyhow::Result<()> {
    let mut framed = Framed::new(stream, KvCommandCodec);

    loop {
        match framed.next().await {
            Some(Ok(cmd)) => {
                println!("received cmd: {:?}", cmd);
                let cmd_resp = handle_cmd(cmd, kv_store).await?;

                println!("send resp: {:?}", cmd_resp);
                framed.send(cmd_resp).await?;
                break;
            }
            Some(Err(e)) => return Err(e),
            None => return Ok(()),
        }
    }
    Ok(())
}

impl Encoder<KvCommandResp> for KvCommandCodec {
    type Error = anyhow::Error;

    fn encode(
        &mut self,
        item: KvCommandResp,
        dst: &mut bytes::BytesMut,
    ) -> Result<(), Self::Error> {
        let mut buf = BytesMut::with_capacity(4096);
        item.encode(&mut buf)?;

        dst.extend_from_slice(&buf);
        Ok(())
    }
}

impl Decoder for KvCommandCodec {
    type Item = KvCommand;
    type Error = anyhow::Error;

    fn decode(&mut self, src: &mut bytes::BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        match KvCommand::decode(src) {
            Ok(cmd) => Ok(Some(cmd)),
            Err(e) => Err(e.into()),
        }
    }
}

async fn handle_cmd(cmd: KvCommand, store: &mut KvStore) -> Result<KvCommandResp, anyhow::Error> {
    match cmd.command {
        Some(c) => match c {
            kv_command::Command::Get(get) => {
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

                Ok(KvCommandResp {
                    command_resp: Some(kv_command_resp::CommandResp::GetResp(CmdGetResp {
                        status: status.to_string(),
                        key_val,
                    })),
                })
            }
            kv_command::Command::Set(set) => {
                let kv = set.key_val.unwrap();
                store.set(kv.clone());

                Ok(KvCommandResp {
                    command_resp: Some(kv_command_resp::CommandResp::SetResp(CmdSetResp {
                        status: "ok".to_string(),
                        key_val: Some(kv),
                    })),
                })
            }
            kv_command::Command::Del(del) => {
                store.del(del.key);
                Ok(KvCommandResp {
                    command_resp: Some(kv_command_resp::CommandResp::DelResp(CmdDelResp {
                        status: "ok".to_string(),
                    })),
                })
            }
        },
        None => Ok(KvCommandResp { command_resp: None }),
    }
}
