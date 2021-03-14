use std::path::PathBuf;
use structopt::StructOpt;
use dialoguer::{
    Select,
    theme::{ColorfulTheme, Theme},
};
use console::Term;
use std::format;
use enum_iterator::IntoEnumIterator;

#[derive(StructOpt, Debug)]
struct Opt{
    // a flag, true if used in the command line

    #[structopt(short, long)]
    dubug: bool,

    // The number of occurerences of the 'v/verbose' flag
    #[structopt(short, long, parse(from_occurrences))]
    verbose: u8,

    // Select Serial port
    #[structopt(short = "p", long = "port")]
    port: Option<String>,

    // Select config file
    #[structopt(short = "c", long = "config", parse(from_os_str))]
    config_file: Option<PathBuf>,
    //
    // Output file
    #[structopt(short = "o", long = "output", parse(from_os_str))]
    output: Option<PathBuf>,
}

fn main() -> std::io::Result<()>{

    let term = Term::stdout();
    let opt = Opt::from_args();

    //println!("{:#?}", opt);

    term.clear_screen()?;
    term.set_title("Serial logger");

    let opt_str = format!("{:#?}", opt);
    term.write_line(&opt_str)?;


    let selection = Select::with_theme(&ColorfulTheme::default())
        .item("Inport config file")
        .item("Create config file")
        .interact()?;
    //replace with match statement
    println!("\"More usefull text\t\"{}", selection);

    match selection {
        0 => import_config(),
        1 => make_config(term)?,
        _ => unreachable!("Out of menue"),
    }

    Ok(())
}

#[derive(Default)]
struct State{
    port: Option<String>,
    baud: Option<String>,
    file: Option<String>,
}


#[derive(IntoEnumIterator, PartialEq, Eq)]
#[repr(usize)]
enum Options{
    Port = 0,
    Baud,
    File,
    Exit,
}
impl Options{
    fn map(&self, state: &State) -> String{
        use Options::*;
        match self {
            Port => {
                state.port
                    .as_ref()
                    .map(|p| format!("Change current port: ({})", p))
                    .unwrap_or_else(|| format!("Set port"))
            }
            Baud => {
                state.baud
                    .as_ref()
                    .map(|p| format!("Change current baud rate: ({})", p))
                    .unwrap_or_else(|| format!("Set baud rate"))
            }
            File => {
                state.file
                    .as_ref()
                    .map(|p| format!("Change output file location: ({})", p))
                    .unwrap_or_else(|| format!("Set output file location"))
            }
            Exit =>{
                format!("Save")
            }
        }
    }

    fn make_selection<'s,'t>(state: &'s State, theme: &'t dyn Theme) -> Select<'t>{
        let mut selection = Select::with_theme(theme);

        selection.default(0);

        Self::into_enum_iter()
            // Set each one to their value based off state
            .map(|s| s.map(&state))
            // Add each item into selection
            .fold(selection, |mut s,a| {s.item(a); s})
    }

    fn update_state(options: usize, state: &mut State, term: &Term) -> bool{
        if options == Self::Port as usize{
                term.write_line("Enter port name: ");
                term.move_cursor_up(1);
                term.move_cursor_right(17);
                state.port = term.read_line().ok();
        }
        else if options == Self::Baud as usize{
                term.write_line("Enter Baud Rate: ");
                term.move_cursor_up(1);
                term.move_cursor_right(17);
                state.baud = term.read_line().ok();
        }
        else if options == Self::File as usize{
                term.write_line("Enter File Path: ");
                term.move_cursor_up(1);
                term.move_cursor_right(17);
                state.file = term.read_line().ok();
        }
        else if options == Self::Exit as usize{return false;}
        else {unreachable!("Outside of menue");}
        true
    }
}

fn make_config(term: Term) -> std::io::Result<()>{

    let mut state = State::default();

    let theme = ColorfulTheme::default();

    let mut should_cont = true;

    while should_cont{

        let opt = Options::make_selection(&state, &theme).interact()?;

        should_cont = Options::update_state(opt, &mut state, &term);

    }





    /*
       let mut port = None;
       let mut baud = None;
       loop{
       let selection = Select::with_theme(&ColorfulTheme::default())
       .item(State.port)
       .item("Set Baud rate")
       .item("Parity bit?")
       .item("Log file location")
       .item("Finish config")
       .interact()?;

       match selection {
       0 => {
       term.write_line("Enter port name: ")?;
       let mut port_t = String::new();
       io::stdin()
       .read_line(&mut port_t)
       .expect("Failed to read port");
       port_t.pop();
       port.replace(port_t);
       }
       1 => {
       term.write_line("Enter baud rate: ")?;
       let mut baud = String::new();
       io::stdin()
       .read_line(&mut baud)
       .expect("Failed to read baud");
       }
       4 => break,
       _ => unreachable!("out of menue"),
       }

       }
       */
    Ok(())
}

fn import_config(){

}
