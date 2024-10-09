use ipc_channel::ipc::{IpcOneShotServer, IpcSender};
use ipc_channel_practice::Payload;

fn main() {
    let tx_server_name = std::env::args().nth(1).unwrap();

    println!("connecting to server");
    let tx: IpcSender<Payload> = IpcSender::connect(tx_server_name).unwrap();

    // create a connection of opposite direction
    let (rx, rx_server_name) = IpcOneShotServer::<Payload>::new().unwrap();
    tx.send(Payload::Connect(rx_server_name)).unwrap();

    let (_, data) = rx.accept().unwrap();
    println!("{data:?}");
}
