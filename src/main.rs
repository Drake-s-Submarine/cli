use structopt::StructOpt;
use std::os::unix::net::UnixStream;
use std::io::Write;
use common::commands::*;
use common::commands::serde::*;

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
    

    let cmd: [u8; COMMAND_BUFFER_SIZE] = match opt.cmd.as_str() {
        "bal" => create_ballast_command(opt.a, opt.b).unwrap(),
        "light" => create_light_command(opt.a, opt.b).unwrap(),
        "prop" => create_prop_command(opt.a, opt.b).unwrap(),

        "bad-buf-start" => {
            let mut buf = create_ballast_command(opt.a, opt.b).unwrap();
            buf[0] = 0xB;
            buf
        },
        "bad-buf-end" => {
            let mut buf = create_ballast_command(opt.a, opt.b).unwrap();
            buf[COMMAND_BUFFER_SIZE - 1] = 0xE;
            buf
        },
        "bad-buf-mod" => {
            let mut buf = create_ballast_command(opt.a, opt.b).unwrap();
            buf[1] = 0xF;
            buf
        },
        _ => {
            println!("Malformed command: {}", opt.cmd);
            return;
        }
    };

    println!("{:X?}", cmd);

    socket.write_all(&cmd)
        .expect("Failed to write command to socket.");
}

fn create_ballast_command(active: f32, mode: f32) -> Result<[u8; COMMAND_BUFFER_SIZE], ()> {
    let command = if active > 0.5 {
        if mode > 0.5 {
            BallastCommand::Intake
        } else {
            BallastCommand::Discharge
        }
    } else {
        BallastCommand::Idle
    };

    Ok(command.serialize())
}

fn create_light_command(x: f32, _y: f32) -> Result<[u8; COMMAND_BUFFER_SIZE], ()> {
    let command = match x as u32 {
        0 => LightCommand::Off,
        1 => LightCommand::On,
        2 => LightCommand::Blink,
        _ => return Err(())
    };

    Ok(command.serialize())
}

fn create_prop_command(x: f32, y: f32) -> Result<[u8; COMMAND_BUFFER_SIZE], ()> {
    let mut x = x;
    if x > 1.0 && x <= 2.0 {
        x = x - 1.0;
        x = x * (-1.0);
    };
    let command = PropulsionCommand::SetThrust(DirectionVector{x, y});

    Ok(command.serialize())
}
