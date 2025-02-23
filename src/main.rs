//! Read agenda items from user prompts
//! and generate an html table
//! formatted per our standards
use clap::Parser;
use std::error::Error;
use std::io::Write;

mod cli;
use crate::cli::Args;

#[allow(dead_code)]
#[derive(Debug)]
struct AgendaItem {
    time: String,
    subject: String,
    presenter: String,
}

const INTRO: &str = r#"
<div class="table_component">
<table>
    <caption>
        <p><strong>Agenda</strong></p>
    </caption>
<thead>
<tr>
    <th>Time</th>
    <th>Subject</th>
    <th>Presenter</th>
</tr>
</thead>
<tbody>
"#;

fn inquire(prompt: String) -> String {
    let mut line = String::new();
    println!("{}: ", prompt);
    let _b1 = std::io::stdin().read_line(&mut line).unwrap();
    // by default the input string includes the Enter key (newline character)
    // we remove it here with trim() and ensure we return a string
    line.trim().to_string()
}

//fn get_agenda_item() -> Vec<AgendaItem> {
//    let mut agenda: Vec<AgendaItem> = Vec::new();
//    loop {
//        let t = inquire(String::from("Time"));
//        if t.is_empty() {
//            break;
//        }
//        let s = inquire(String::from("Subject"));
//        let p = inquire(String::from("Presenter"));
//        let item = AgendaItem {
//            time: t,
//            subject: s,
//            presenter: p,
//        };
//        agenda.push(item);
//    }
//    agenda
//}

fn get_agenda_items_interactively(agenda: &mut Vec<AgendaItem>) {
    loop {
        let t = inquire(String::from("Time"));
        if t.is_empty() {
            break;
        }
        let s = inquire(String::from("Subject"));
        let p = inquire(String::from("Presenter"));
        let item = AgendaItem {
            time: t,
            subject: s,
            presenter: p,
        };
        agenda.push(item);
    }
}

fn get_outfile(output_filename: Option<&std::path::PathBuf>) -> Box<dyn std::io::Write> {
    match output_filename {
        Some(filename) => Box::new(std::io::BufWriter::new(
            std::fs::File::create(filename).unwrap_or_else(|err| {
                panic!("Error {err}. Unable to open {}", filename.to_string_lossy())
            }),
        )),
        None => Box::new(std::io::stdout().lock()),
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Table maker");
    let args = Args::parse();
    println!("Output: {:?}", args.output.as_deref());
    println!("Input: {:?}", args.input.as_deref());
    println!("Verbose: {:?}", args.verbose);

    //let output_file = Some(std::path::PathBuf::from("src/agenda.html"));
    //let mut file = get_outfile(output_file.as_ref());
    let mut file = get_outfile(args.output.as_ref());

    let mut agenda: Vec<AgendaItem> = Vec::new();
    get_agenda_items_interactively(&mut agenda);

    let path_to_style = Some(String::from("src/style.html"));
    let style = std::fs::read_to_string(
        path_to_style.ok_or_else(|| String::from("Can't read stylme.html"))?,
    )?;

    write!(&mut file, "{}", style)?;
    write!(&mut file, "{}", INTRO)?;
    for i in agenda {
        writeln!(&mut file, "<tr>")?;
        writeln!(&mut file, "    <td>{}</td>", i.time)?;
        writeln!(&mut file, "    <td>{}</td>", i.subject)?;
        writeln!(&mut file, "    <td>{}</td>", i.presenter)?;
        writeln!(&mut file, "</tr>")?;
    }
    writeln!(&mut file, "</tbody>\n</table>\n</div>\n")?;

    Ok(())
}
