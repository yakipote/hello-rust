use std::env;
#[macro_use]
extern crate log;

// mod tcp_client;
mod tcp_server;
// mod udp_client;
// mod udp_server;
fn main() {
    env::set_var("RUST_LOG","debug");
    env_logger::init();
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        error!("Please specify [tcp|udp] [server|client] [addr:port]");
        std::process::exit(1);
    }
    let protocol: &str = &args[1];
    let role: &str = &args[2];
    let address: &str = &args[3];

    match protocol  {
       "tcp" => match role{
           "server" => {
               tcp_server::serve(address).unwrap_or_else(|e| error!("{}",e));
           }
           "client" => {
               // TODO: TCPクライアントの呼び出し
           }
           _ => {
               // missing_role();
           }
       },
        "udp" => match role {
            "server" => {
                // TODO: UDPサーバの呼び出し
            }
            "client" => {
                // TODO: UDPクライアントの呼び出し
            }
            _ => {
                // missing_role();
            }
        },
        _ => {
            error!("Please specify tcp or udp on the 1st argument.");
        }
    }
}
