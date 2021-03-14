use std::io;
use std::path::PathBuf;
use structopt::StructOpt;
use dialoguer::{
    Select,
    theme::ColorfulTheme
};
use console::Term;
use std::format;

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
        .item("Creat config file")
        .interact()?;
    //replace with match statement
    println!("\"More usefull text\t\"{}", selection);

    match selection {
        0 => import_config(),
        1 => make_config(term)?,
        _ => unreachable!("Out of menue"),
    }


    println!("Select file");
    let mut file = String::new();

    io::stdin()
        .read_line(&mut file)
        .expect("Failed to read line");

    println!("You selected file:\n{}",file);

    Ok(())
}


fn make_config(term: Term) -> std::io::Result<()>{
    let selection = Select::with_theme(&ColorfulTheme::default())
        .item("Set port")
        .item("Set Baud rate")
        .item("Paridy bit?")
        .item("Log file location")
        .interact()?;

    match selection {
        0 => {
            term.write_line("Enter port name: ")?;
            let mut port = String::new();
            io::stdin()
                .read_line(&mut port)
                .expect("Failed to read port");
            },
        1 => {
            term.write_line("Enter baud rate: ")?;
            let mut baud = String::new();
            io::stdin()
                .read_line(&mut baud)
                .expect("Failed to read baud");
            },
        _ => unreachable!("out of menue"),
        }

    Ok(())
}

fn import_config(){

}
