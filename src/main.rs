use clap::Parser;
use console::Term;
use dialoguer::{
    theme::{ColorfulTheme, Theme},
    Select,
};
use log::{error, info};
use std::{format, fs::File, path::PathBuf};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

//
#[derive(Debug, Parser)]
struct Args {
    // a flag, true if used in the command line
    #[arg(short, long)]
    dubug: bool,

    #[command(flatten)]
    verbose: clap_verbosity_flag::Verbosity,

    // Select Serial port
    #[arg(short, long = "port", requires = "baud")]
    port: Option<String>,

    #[arg(short, long = "baud", requires = "port")]
    baud: Option<u8>,

    // Select config file
    #[arg(short, long = "config", default_value = "config.yaml")]
    config_file: PathBuf,
    //
    // Output file
    #[arg(short, long = "output", default_value = "output.csv")]
    output_file: PathBuf,
}

// Takes upened file and writes standard config info to it.
fn create_config(output: File) -> File {
    //todo put in yaml code to make basic config
    println!("Create_config");
    output
}

fn is_valid_config(config: File) -> bool {
    false
}

fn main() -> std::io::Result<()> {
    let term = Term::stdout();
    let args = Args::parse();

    // Open/create config file
    let config = match File::open(&args.config_file) {
        Ok(config) => {
            info!("Config file found");
            config
        }
        Err(_) => {
            info!("Creating config file");
            File::create(&args.config_file)?;
            //create_config(File::open(&args.config_file)?)
            File::open(&args.config_file).map(create_config)?
        } //set config menu flag
    };

    // Open/create output file
    let output_file = match File::open(&args.output_file) {
        Ok(output) => output,
        Err(_) => File::create(&args.output_file)?,
    };

    //println!("{:#?}", args);

    //term.clear_screen()?;
    term.set_title("Serial logger");

    if !is_valid_config(config) {
        term.write_line("Enter Port info")?;
        make_config(&term)?;
    }
    //let opt_str = format!("{:#?}", args);
    // write debug macro
    // term.write_line(&opt_str)?;

    let selection = Select::with_theme(&ColorfulTheme::default())
        .default(0)
        .item(format!(
            "Run with current config: {}",
            args.config_file.to_str().unwrap()
        ))
        .item("Inport config file")
        .item("Edit config file")
        .interact()?;
    //replace with match statement
    println!("\"More usefull text\t\"{}", selection);

    match selection {
        0 => import_config(false),
        1 => import_config(true),
        2 => make_config(&term)?,
        _ => unreachable!("Out of menue"),
    }

    Ok(())
}

#[derive(Default)]
struct State {
    port: Option<String>,
    baud: Option<String>,
    file: Option<String>,
}

#[derive(PartialEq, Eq, EnumIter)]
#[repr(usize)]
enum Options {
    Port = 0,
    Baud,
    File,
    Exit,
}
impl Options {
    fn map(&self, state: &State) -> String {
        use Options::*;
        match self {
            Port => state
                .port
                .as_ref()
                .map(|p| format!("Change current port: ({})", p))
                .unwrap_or_else(|| format!("Set port")),
            Baud => state
                .baud
                .as_ref()
                .map(|p| format!("Change current baud rate: ({})", p))
                .unwrap_or_else(|| format!("Set baud rate")),
            File => state
                .file
                .as_ref()
                .map(|p| format!("Change output file location: ({})", p))
                .unwrap_or_else(|| format!("Set output file location")),
            Exit => {
                format!("Save")
            }
        }
    }

    fn make_selection<'s, 't>(state: &'s State, theme: &'t dyn Theme) -> Select<'t> {
        let mut selection = Select::with_theme(theme);

        selection.default(0);

        Self::iter()
            // Set each one to their value based off state
            .map(|s| s.map(&state))
            // Add each item into selection
            .fold(selection, |mut s, a| {
                s.item(a);
                s
            })
    }

    fn update_state(options: usize, state: &mut State, term: &Term) -> bool {
        if options == Self::Port as usize {
            term.write_line("Enter port name: ").ok();
            term.move_cursor_up(1).ok();
            term.move_cursor_right(17).ok();
            state.port = term.read_line().ok();
        } else if options == Self::Baud as usize {
            term.write_line("Enter Baud Rate: ").ok();
            term.move_cursor_up(1).ok();
            term.move_cursor_right(17).ok();
            state.baud = term.read_line().ok();
        } else if options == Self::File as usize {
            term.write_line("Enter File Path: ").ok();
            term.move_cursor_up(1).ok();
            term.move_cursor_right(17).ok();
            state.file = term.read_line().ok();
        } else if options == Self::Exit as usize {
            return false;
        } else {
            unreachable!("Outside of menue");
        }
        true
    }
}

fn make_config(term: &Term) -> std::io::Result<()> {
    let mut state = State::default();

    let theme = ColorfulTheme::default();

    let mut should_cont = true;

    while should_cont {
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

fn import_config(default: bool) {}
