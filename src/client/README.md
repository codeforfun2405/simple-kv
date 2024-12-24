
# Client

## Get

```sh
simple-kv (main)> cargo run --bin client get test

received resp: KvCommandResp { command_resp: Some(GetResp(CmdGetResp { status: "ok", key_val: Some(KeyVal { key: "test", value: b"test-value" }) })) }
```

## Set

```sh
simple-kv (main)> cargo run --bin client set hello good-morning

received resp: KvCommandResp { command_resp: Some(SetResp(CmdSetResp { status: "ok", key_val: Some(KeyVal { key: "hello", value: b"good-morning" }) })) }

simple-kv (main)> cargo run --bin client get hello

received resp: KvCommandResp { command_resp: Some(GetResp(CmdGetResp { status: "ok", key_val: Some(KeyVal { key: "hello", value: b"good-morning" }) })) }


simple-kv (main)> cargo run --bin client set date 20224-12-24

received resp: KvCommandResp { command_resp: Some(SetResp(CmdSetResp { status: "ok", key_val: Some(KeyVal { key: "date", value: b"20224-12-24" }) })) }

simple-kv (main)> cargo run --bin client get date

received resp: KvCommandResp { command_resp: Some(GetResp(CmdGetResp { status: "ok", key_val: Some(KeyVal { key: "date", value: b"20224-12-24" }) })) }
```

## Display Kv

```sh
/simple-kv (main)> cargo run --bin client get date

key: date, value: b"20224-12-24"
```
