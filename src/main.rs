use structopt::StructOpt;
use std::os::unix::net::UnixStream;
use std::io::Write;

// TODO: create shared definitions

const BALLAST_ID: u8 = 0x0;
const BUFFER_BYTE_LEN: usize = 16;

#[derive(StructOpt, Debug)]
#[structopt(name = "Sub Command Utility",
            about = "A cli for testing submarine commands.")]
struct Opt {
    cmd: String,
}

fn main() {
    let opt = Opt::from_args();
    let mut socket = UnixStream::connect("/tmp/sub_cmd_socket")
        .expect("Failed to connect to socket.");
    
    let cmd: [u8; BUFFER_BYTE_LEN] = match opt.cmd.as_str() {
        "enable" => create_ballast_command(1).unwrap(),
        "disable" => create_ballast_command(0).unwrap(),
        "bad-buf-start" => {
            let mut buf = create_ballast_command(0).unwrap();
            buf[0] = 0xB;
            buf
        },
        "bad-buf-end" => {
            let mut buf = create_ballast_command(0).unwrap();
            buf[BUFFER_BYTE_LEN - 1] = 0xE;
            buf
        },
        "bad-buf-mod" => {
            let mut buf = create_ballast_command(0).unwrap();
            buf[1] = 0xF;
            buf
        },
        _ => {
            println!("Malformed command: {}", opt.cmd);
            return;
        }
    };

    socket.write_all(&cmd)
        .expect("Failed to write command to socket.");
}

fn create_command_buffer_template() -> [u8; BUFFER_BYTE_LEN] {
    let mut buf: [u8; BUFFER_BYTE_LEN] = [0; BUFFER_BYTE_LEN];

    buf[0] = 0xA;
    buf[BUFFER_BYTE_LEN - 1] = 0xF;

    buf
}

fn create_ballast_command(state: u8) -> Result<[u8; BUFFER_BYTE_LEN], ()> {
    let mut buf = create_command_buffer_template();

    if state < 3 {
        buf[2] = state;
        buf[1] = BALLAST_ID;
        Ok(buf)
    } else {
        Err(())
    }
}
