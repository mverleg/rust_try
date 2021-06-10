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
    #[structopt(short = "l", long = "lines", help = "Maximum number of lines to show.")]
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
            .title("Please fill out this form")
            .button("Ok", |s| s.quit())
            .content(
                ListView::new()
                    // Each child is a single-line view with a label
                    .child("Name", EditView::new().fixed_width(10))
                    .child("Presentation", TextArea::new().min_height(4))
                    .child(
                        "Receive spam?",
                        Checkbox::new().on_change(|s, checked| {
                            // Enable/Disable the next field depending on this checkbox
                            for name in &["email1", "email2"] {
                                s.call_on_name(name, |view: &mut EditView| {
                                    view.set_enabled(checked)
                                });
                                if checked {
                                    s.focus_name("email1").unwrap();
                                }
                            }
                        }),
                    )
                    .child(
                        "Email",
                        // Each child must have a height of 1 line,
                        // but we can still combine multiple views!
                        LinearLayout::horizontal()
                            .child(
                                EditView::new()
                                    .disabled()
                                    .with_name("email1")
                                    .fixed_width(15),
                            )
                            .child(TextView::new("@"))
                            .child(
                                EditView::new()
                                    .disabled()
                                    .with_name("email2")
                                    .fixed_width(10),
                            ),
                    )
                    // Delimiter currently are just a blank line
                    .delimiter()
                    .child(
                        "Age",
                        // Popup-mode SelectView are small enough to fit here
                        SelectView::new()
                            .popup()
                            .item_str("0-18")
                            .item_str("19-30")
                            .item_str("31-40")
                            .item_str("41+"),
                    )
                    .with(|list| {
                        // We can also add children procedurally
                        for i in 0..50 {
                            list.add_child(
                                &format!("Item {}", i),
                                EditView::new(),
                            );
                        }
                    })
                    .scrollable(),
            ),
    );

    siv.run();
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
