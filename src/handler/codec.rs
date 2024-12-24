use crate::pb::kv::{KvCommand, KvCommandResp};
use anyhow::Result;
use bytes::BytesMut;
use prost::Message;
use tokio_util::codec::{Decoder, Encoder};

// parse the data into cmd
#[derive(Debug)]
pub struct KvCommandCodec;

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
