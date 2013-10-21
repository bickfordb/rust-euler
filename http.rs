extern mod extra;

use extra::comm::{stream, Port};
use extra::net::ip;
use extra::net::tcp;
use extra::uv;
use std::comm;
use std::io;
use std::result;
use std::task;

type ConnectMsg = (tcp::TcpNewConnection, comm::SharedChan<Option<tcp::TcpErrData>>);


fn main() {
  //Connection information will be transmitted using this Port and Chan
  let (port, chan): (Port<ConnectMsg>, Chan<ConnectMsg>) = stream();

  // this is the task which accepts new connections
  do task::spawn {
    loop {
      let (conn, kill_ch) = port.recv();
      do task::spawn {
        io::println("Going to accept a new connection");
        match tcp::accept(conn) {
          result::Err(err) => {
            io::println(fmt!("Connection error: %?", err));
            kill_ch.send(Some(err));
          },
          result::Ok(socket) => {
            let peer_addr: ~str = ip::format_addr(&socket.get_peer_addr());
            io::println(fmt!("Connection accepted from %s", peer_addr));

            loop {
              let result = socket.read(0u);
              if result.is_err() {
                break;
              }

              let c = result.get();
              io::println(fmt!("%?", c));
              let result = socket.write(c);
              if result.is_err() {
                break;
              }
            }
            io::println(fmt!("Connection closed from %s", peer_addr));
          }
        }
      }
    }
  }

  let result = tcp::listen(ip::v4::parse_addr("127.0.0.1"), 4000, 100,
      &uv::global_loop::get(), |_|{
        io::println("Server is now listening!");
      },
      |conn, kill_chan| {
        // The connection must be accepted from another task or the server will block.
        chan.send((conn, kill_chan));
      }
      );

      if result.is_err() {
        fail!(fmt!("failed listen: %?", result.get_err()));
      }
    }
