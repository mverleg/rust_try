use ::std::io::{Stdout, stdout};
use ::std::thread::sleep;
use ::std::time::Duration;

use ::crossterm::{cursor, QueueableCommand};
use ::crossterm::Result;
use ::crossterm::terminal;
use ::structopt::StructOpt;
use ::std::io::Write;
use ::std::env::current_dir;

use ::whoami::username;
use ::whoami::hostname;

#[derive(Debug, StructOpt)]
#[structopt(name = "termchat", about = "Welcome to chat in your console!")]
struct Args {
    #[structopt(subcommand)]
    cmd: Option<Cmd>,
}

#[derive(Debug, StructOpt, Default)]
struct OpenArgs {
    #[structopt(short = "l", long = "lines", default_value = "0", help = "Maximum number of lines to show.")]
    lines: u16,
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

fn clear_line(out: &mut Stdout, width: u16) -> Result<()> {
    out.queue(cursor::MoveToColumn(0))?;
    print!("{}", " ".repeat((width - 1) as usize));
    out.queue(cursor::MoveDown(1))?;
    out.queue(cursor::MoveToColumn(0))?;
    Ok(())
}

fn save_lines(out: &mut Stdout, width: u16, height: u16) -> Result<()> {
    out.queue(cursor::MoveUp(1))?;
    clear_line(out, width)?;
    print!("{}", "\n".repeat(height as usize));
    out.queue(cursor::MoveUp(height + 1))?;
    out.flush()
}

fn get_prompt() -> String {
    let dir = current_dir()
        .map(|mut pth| pth.file_name().unwrap())
        .unwrap_or_else("msg");
    format!("{}@{} {}$ ", username(), hostname(), dir)
}

fn show_chat(out: &mut Stdout, lines: u16) -> Result<()> {
    let (width, height) = terminal::size()?;
    let lines = if lines == 0 || lines > height {
        height - 5
    } else {
        lines
    };
    //TODO @mark: rename
    save_lines(out, width, lines)?;
    //TODO @mark: use
    get_prompt()

    // for _ in 0..lines {
    //     stdout()
    //         .queue(cursor::(1))?
    //         .flush()?;
    //     sleep(Duration::from_millis(200));
    // }

    Ok(())
}

fn main() {
    let args = Args::from_args();
    let cmd = args.cmd.unwrap_or_else(|| Cmd::Open(OpenArgs::default()));
    let mut out = stdout();
    sleep(Duration::from_millis(200));
    match cmd {
        Cmd::Open(_) => unimplemented!(),
        Cmd::One(open) => show_chat(&mut out, open.lines),
        Cmd::RunDaemon => unimplemented!(),
        Cmd::HasMessage => unimplemented!(),
    }.expect("an error happened");
    sleep(Duration::from_millis(400));
}
