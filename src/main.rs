use structopt::StructOpt;
use std::os::unix::net::UnixStream;
use std::io::Write;

#[derive(StructOpt, Debug)]
#[structopt(name = "Sub Command Utility",
            about = "A cli for testing submarine commands.")]
struct Opt {
    cmd: String,
}

fn main() {
    println!("Reading args");
    let opt = Opt::from_args();
    println!("Binding socket");
    let mut socket = UnixStream::connect("/tmp/sub_cmd_socket")
        .expect("Failed to connect to socket.");
    println!("Socket bound!");
    
    let cmd = match opt.cmd.as_str() {
        "enable" => "EnableMotor",
        "disable" => "DisableMotor",
        _ => {
            println!("Malformed command: {}", opt.cmd);
            return
        }
    };

    println!("Command is: {}", cmd);

    socket.write_all(cmd.as_bytes())
        .expect("Failed to write command to socket.");

    //for stream in socket.incoming() {
    //    println!("TEST");
    //    match stream {
    //        Ok(mut stream) => {
    //            println!("Writing command");
    //            stream.write(cmd.as_bytes()).expect("Failed to write command to socket.");
    //            println!("Done");
    //            stream.shutdown(Shutdown::Write).expect("Failed to shut down socket.");
    //        }
    //        Err(e) => println!("Failed to connect to stream: {}", e)
    //    }
    //}
}
