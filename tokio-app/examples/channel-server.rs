use bytes::Bytes;
use mini_redis::client;
use tokio::sync::{mpsc, oneshot};

#[derive(Debug)]
enum ChannelCommand {
  Get {
    key: String,
    resp: Responder<Option<Bytes>>,
  },
  Set {
    key: String,
    val: Vec<u8>,
    resp: Responder<()>,
  },
}

type Responder<T> = oneshot::Sender<mini_redis::Result<T>>;

#[tokio::main]
async fn main() {
  let (tx, mut rx) = mpsc::channel(32);
  let tx2 = tx.clone();

  let manager = tokio::spawn(async move {
    let mut client = client::connect("127.0.0.1:6379").await.unwrap();

    while let Some(cmd) = rx.recv().await {
      match cmd {
        ChannelCommand::Get { key, resp } => {
          let res = client.get(&key).await;
          let _ = resp.send(res);
        }
        ChannelCommand::Set { key, val, resp } => {
          let res = client.set(&key, val.into()).await;
          let _ = resp.send(res);
        }
      }
    }
  });

  let t1 = tokio::spawn(async move {
    let (resp_tx, resp_rx) = oneshot::channel();
    let cmd = ChannelCommand::Get {
      key: "hello".to_string(),
      resp: resp_tx,
    };
    tx.send(cmd).await.unwrap();
    let res = resp_rx.await;
    println!("GOT = {:?}", res);
  });

  let t2 = tokio::spawn(async move {
    let (resp_tx, resp_rx) = oneshot::channel();
    let cmd = ChannelCommand::Set {
      key: "foo".to_string(),
      val: "bar".into(),
      resp: resp_tx,
    };
    tx2.send(cmd).await.unwrap();
    let res = resp_rx.await;
    println!("GOT = {:?}", res);
  });

  t1.await.unwrap();
  t2.await.unwrap();
  manager.await.unwrap();
}
