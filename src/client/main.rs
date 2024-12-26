use bytes::Bytes;
use futures::SinkExt;
use futures::StreamExt;
use simple_kv::pb::kv::kv_command::Command;
use simple_kv::pb::kv::kv_command_resp::CommandResp;
use simple_kv::{handler::client_codec, pb::kv::KvCommand};

use tokio::net::TcpStream;
use tokio_util::codec::Framed;
use tokio_util::codec::LinesCodec;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // let args = std::env::args().collect::<Vec<String>>();
    // if args.len() < 2 {
    //     println!("usage: {} <set|get>", args[0]);
    //     return Ok(());
    // }

    let addrs = "127.0.0.1:8088";
    let stream = TcpStream::connect(&addrs).await?;

    // let mut framed = Framed::new(stream, client_codec::KvCommandCodec);
    let mut framed = Framed::new(stream, LinesCodec::new());

    // match args[1].as_str() {
    //     "set" => {
    //         let bs = args[3].as_bytes().to_vec();
    //         set(&mut framed, args[2].clone(), bs.into()).await?
    //     }
    //     "get" => get(&mut framed, args[2].to_string()).await?,
    //     _ => println!("unknown command"),
    // }

    for cmd in build_set_commands() {
        let kv_cmd = KvCommand::new(Some(cmd));
        println!("sending cmd: {:?}", kv_cmd);

        framed.send(kv_cmd).await?;
    }
    framed.flush().await?;

    let h = tokio::spawn(async move {
        match receive_resp(&mut framed).await {
            Ok(_) => {
                println!("receive_resp done");
            }
            Err(e) => {
                println!("have error: {}", e);
            }
        }
    });

    h.await?;

    // for cmd in build_get_commands() {
    //     let kv_cmd = KvCommand::new(Some(cmd));
    //     framed.send(kv_cmd).await?;
    // }

    Ok(())
}

fn build_set_commands() -> Vec<Command> {
    let mut commands = Vec::new();

    commands.push(Command::new_set("key1".to_string(), b"hello".to_vec()));
    commands.push(Command::new_set("key2".to_string(), b"world!".to_vec()));
    commands.push(Command::new_set("key3".to_string(), b"Good".to_vec()));
    commands.push(Command::new_set("key6".to_string(), b"Morning".to_vec()));
    commands
}

fn build_get_commands() -> Vec<Command> {
    let mut commands = Vec::new();

    commands.push(Command::new_get("key1".to_string()));
    commands.push(Command::new_get("key2".to_string()));
    commands.push(Command::new_get("key3".to_string()));
    commands.push(Command::new_get("key6".to_string()));
    commands.push(Command::new_get("key5".to_string()));
    commands
}

async fn set(
    stream: &mut Framed<TcpStream, client_codec::KvCommandCodec>,
    key: String,
    val: Bytes,
) -> anyhow::Result<()> {
    let cmd = KvCommand::new(Some(Command::new_set(key.clone(), val.to_vec())));
    stream.send(cmd).await?;

    while let Some(Ok(resp)) = stream.next().await {
        if resp.command_resp.is_none() {
            break;
        } else {
            println!("received resp: {:?}", resp);
        }
    }

    Ok(())
}

async fn receive_resp(
    stream: &mut Framed<TcpStream, client_codec::KvCommandCodec>,
) -> anyhow::Result<()> {
    while let Some(Ok(resp)) = stream.next().await {
        if resp.command_resp.is_none() {
            break;
        } else {
            println!("received resp: {:?}", resp);
        }
    }
    Ok(())
}

async fn get(
    stream: &mut Framed<TcpStream, client_codec::KvCommandCodec>,
    key: String,
) -> anyhow::Result<()> {
    let cmd = KvCommand::new(Some(Command::new_get(key)));
    stream.send(cmd).await?;

    while let Some(Ok(resp)) = stream.next().await {
        if resp.command_resp.is_none() {
            break;
        } else {
            match resp.command_resp.unwrap() {
                CommandResp::GetResp(get_resp) => {
                    if let Some(kv) = get_resp.key_val {
                        println!("{}", kv);
                    }
                }
                _ => {}
            }
        }
    }
    Ok(())
}
