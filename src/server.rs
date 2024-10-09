use ipc_channel::ipc::{IpcOneShotServer, IpcSender};
use ipc_channel_practice::Payload;

fn main() {
    let (rx, rx_server_name) = IpcOneShotServer::<Payload>::new().unwrap();
    println!("{rx_server_name}");

    let (_, data) = rx.accept().unwrap();

    let tx: IpcSender<Payload> = match data {
        Payload::Connect(tx_server_name) => {
            println!("connecting to client");
            IpcSender::connect(tx_server_name).unwrap()
        }
        Payload::Message(_) => panic!("Why I got a message?"),
    };

    tx.send(Payload::Message("カラテが高まる".to_string()))
        .unwrap();
}
