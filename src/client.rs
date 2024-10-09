use ipc_channel::ipc::IpcSender;

fn main() {
    let server_name = std::env::args().nth(1).unwrap();
    let tx: IpcSender<Vec<u8>> = IpcSender::connect(server_name).unwrap();
    tx.send(vec![0x10, 0x11, 0x12, 0x13]).unwrap();
}
