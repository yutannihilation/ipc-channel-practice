use ipc_channel::ipc::IpcOneShotServer;

fn main() {
    let (server, server_name) = IpcOneShotServer::new().unwrap();
    println!("{server_name}");

    let (_, data): (_, Vec<u8>) = server.accept().unwrap();
    println!("{data:?}")
}
