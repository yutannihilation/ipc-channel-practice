use ipc_channel::ipc::{IpcOneShotServer, IpcSender};
use ipc_channel_practice::Payload;

fn main() {
    let tx_server_name = std::env::args().nth(1).unwrap();

    println!("connecting to server");
    let tx: IpcSender<Payload> = IpcSender::connect(tx_server_name).unwrap();
    println!("telling server that client is ready");
    tx.send(Payload::Ready).unwrap();

    // create a connection of opposite direction
    let (rx_server, rx_server_name) = IpcOneShotServer::<Payload>::new().unwrap();
    println!("letting server connect to client");
    tx.send(Payload::Connect(rx_server_name)).unwrap();

    println!("waiting for the server is ready");
    let rx = match rx_server.accept() {
        Ok((rx, data)) => {
            if !matches!(data, Payload::Ready) {
                panic!("Got unexpected payload. Expected: Ready, Got: {data:?}")
            }
            rx
        }
        Err(e) => panic!("failed to accept connection: {e}"),
    };

    println!("receiving payload from server");
    let data = rx.recv().unwrap();
    println!("{data:?}");
}
