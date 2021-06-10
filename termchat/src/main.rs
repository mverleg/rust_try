use ::structopt::StructOpt;

//TODO @mark:
use ::cursive::{
    traits::*,
    views::{
        Checkbox, Dialog, EditView, LinearLayout, ListView, SelectView,
        TextArea, TextView,
    },
};
use cursive::CursiveRunnable;

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

fn show_chat(siv: &mut CursiveRunnable, height: u16) {
    siv.add_layer(
        Dialog::new()
            .content(
                ListView::new()
                    .with(|list| {
                        // We can also add children procedurally
                        for i in 0..5 {
                            list.add_child(
                                &format!("Item {}", i),
                                EditView::new(),
                            );
                        }
                    })
                    .scrollable(),
            ),
    );

    eprintln!("before");
    siv.run();
    eprintln!("after");
}

fn main() {
    let args = Args::from_args();
    let cmd = args.cmd.unwrap_or_else(|| Cmd::Open(OpenArgs::default()));
    let mut siv = cursive::default();
    println!("{:?}", cmd);
    match cmd {
        Cmd::Open(_) => unimplemented!(),
        Cmd::One(open) => show_chat(&mut siv, open.lines),
        Cmd::RunDaemon => unimplemented!(),
        Cmd::HasMessage => unimplemented!(),
    }
}
