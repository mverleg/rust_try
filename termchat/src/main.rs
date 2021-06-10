use ::structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "termchat", about = "Welcome to chat in your console!")]
struct Args {
    #[structopt(subcommand)]
    cmd: Option<Cmd>,
}

#[derive(Debug, StructOpt, Default)]
struct OpenArgs {
    #[structopt(short = "l", long = "lines", help = "Number of lines of chat to show.")]
    lines: Option<u16>,
}

#[derive(Debug, StructOpt)]
enum Cmd {
    #[structopt(about = "Start termchat in the main screen (contact list).")]
    Open(OpenArgs),

    #[structopt(about = "Open termchat at the oldest unread message.")]
    One(OpenArgs),

    #[structopt(about = "Start the daemon in blocking mode. This happens automatically.")]
    RunDaemon,

    #[structopt(about = "Exit code 0 is there is a message, 1 if not.")]
    HasMessage,
}

fn main() {
    let args = Args::from_args();
    let cmd = args.cmd.unwrap_or_else(|| Cmd::Open(OpenArgs::default()));
    println!("{:?}", cmd);
    match cmd {
        Open => {},
        One => {},
        RunDaemon => unimplemented!(),
        HasMessage => unimplemented!(),
    }
}
