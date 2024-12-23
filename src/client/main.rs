use bytes::{Bytes, BytesMut};
use prost::Message;
use simple_kv::pb::kv::{kv_command, CmdGet, CmdSet, KeyVal, KvCommand, KvCommandResp};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let addrs = "127.0.0.1:8088";
    let mut tcp_conn = TcpStream::connect(&addrs).await?;

    let bs = b"test".to_vec();
    set(
        &mut tcp_conn,
        "test".to_string(),
        bytes::Bytes::copy_from_slice(&bs),
    )
    .await?;

    // get(tcp_conn).await?;

    Ok(())
}

async fn set(tcp_conn: &mut TcpStream, key: String, val: Bytes) -> anyhow::Result<()> {
    let cmd = KvCommand {
        command: Some(kv_command::Command::Set(CmdSet {
            key_val: Some(KeyVal { key, value: val }),
        })),
    };

    let mut buf = BytesMut::with_capacity(4096);
    cmd.encode(&mut buf)?;
    tcp_conn.write_all(&buf).await?;

    let mut resp_buf = BytesMut::with_capacity(4096);
    tcp_conn.read(&mut resp_buf).await?;

    match KvCommandResp::decode(&mut resp_buf) {
        Ok(resp) => {
            println!("received resp: {:?}", resp);
        }
        Err(e) => {
            println!("have error: {}", e);
        }
    }

    Ok(())
}

async fn get(mut tcp_conn: TcpStream) -> anyhow::Result<()> {
    let mut buf = BytesMut::with_capacity(4096);
    let cmd = KvCommand {
        command: Some(kv_command::Command::Get(CmdGet {
            key: "test".to_string(),
        })),
    };

    cmd.encode(&mut buf)?;
    tcp_conn.write_all(&buf).await?;

    let mut resp_buf = BytesMut::with_capacity(4096);
    tcp_conn.read(&mut resp_buf).await?;

    match KvCommandResp::decode(&mut resp_buf) {
        Ok(resp) => {
            println!("received resp: {:?}", resp);
        }
        Err(e) => {
            println!("have error: {}", e);
        }
    }

    Ok(())
}
