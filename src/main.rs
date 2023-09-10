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
    a: f32,
    b: f32,
}

fn main() {
    let opt = Opt::from_args();
    let mut socket = UnixStream::connect("/tmp/sub_cmd_socket")
        .expect("Failed to connect to socket.");
    
    let cmd: [u8; BUFFER_BYTE_LEN] = match opt.cmd.as_str() {
        "enable" => create_ballast_command(1).unwrap(),
        "disable" => create_ballast_command(0).unwrap(),
        "prop" => create_prop_command(opt.a, opt.b).unwrap(),

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

fn create_prop_command(x: f32, y: f32) -> Result<[u8; BUFFER_BYTE_LEN], ()> {
    let mut buf = create_command_buffer_template();

    let x_bytes = x.to_le_bytes();
    let y_bytes = y.to_le_bytes();

    buf[1] = 0x1;

    buf[2] = x_bytes[0];
    buf[3] = x_bytes[1];
    buf[4] = x_bytes[2];
    buf[5] = x_bytes[3];

    buf[6] = y_bytes[0];
    buf[7] = y_bytes[1];
    buf[8] = y_bytes[2];
    buf[9] = y_bytes[3];

    Ok(buf)
}
