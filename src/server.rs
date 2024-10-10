use ipc_channel::ipc::{IpcOneShotServer, IpcSender};
use ipc_channel_practice::Payload;

fn main() {
    let (rx_server, rx_server_name) = IpcOneShotServer::<Payload>::new().unwrap();
    println!("{rx_server_name}");

    println!("waiting for client is ready");
    let rx = match rx_server.accept() {
        Ok((rx, data)) => {
            if !matches!(data, Payload::Ready) {
                panic!("Got unexpected payload. Expected: Ready, Got: {data:?}")
            }
            rx
        }
        Err(e) => panic!("failed to accept connection: {e}"),
    };

    println!("receiving payload from client");
    let tx: IpcSender<Payload> = match rx.recv().unwrap() {
        Payload::Connect(tx_server_name) => {
            println!("connecting to client");
            IpcSender::connect(tx_server_name).unwrap()
        }
        Payload::Ready => panic!("Why I got ready?"),
        Payload::Message(_) => panic!("Why I got a message?"),
    };

    println!("telling client that server is ready");
    tx.send(Payload::Ready).unwrap();
    println!("sending a message to client");
    tx.send(Payload::Message("カラテが高まる".to_string()))
        .unwrap();
}
