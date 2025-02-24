//! Read agenda items from user prompts
//! and generate an html table
//! formatted per our standards
use clap::Parser;
use inquire::Text;
use std::error::Error;
use std::io::Write;

mod cli;
use crate::cli::Args;

#[allow(dead_code)]
#[derive(Debug, serde::Deserialize)]
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

const STYLE: &str = r#"
<style>
.table_component {
    overflow: auto;
    width: 100%;
    font-family: Arial, Helvetica, sans-serif;
    font-size: 16px;
}
.table_component table {
    border: 1px solid #dededf;
    height: 100%;
    width: 95%;
    table-layout: auto;
    border-collapse: collapse;
    border-spacing: 1px;
    text-align: left;
    margin-left: auto;
    margin-right: auto;
    overflow-x: auto;
}
.table_component caption {
    caption-side: top;
    text-align: center;
}
.table_component th {
    border: 1px solid #dededf;
    background-color: #eceff1;
    color: #000000;
    padding: 8px;
}
.table_component td {
    border: 1px solid #dededf;
    padding: 8px;
}
.table_component tr:nth-child(even) td {
    background-color: #f1e9e9;
    color: #000000;
}
.table_component tr:nth-child(odd) td {
    background-color: #ffffff;
    color: #000000;
}
</style>
"#;

/*
fn inquire(prompt: String) -> String {
    let mut line = String::new();
    println!("{}: ", prompt);
    let _b1 = std::io::stdin().read_line(&mut line).unwrap();
    // by default the input string includes the Enter key (newline character)
    // we remove it here with trim() and ensure we return a string
    line.trim().to_string()
}
*/

fn ask(prompt: String, helpmsg: String) -> String {
    let status = Text::new(&prompt).with_help_message(&helpmsg).prompt();
    status.unwrap()
}

fn get_agenda_items_interactively(agenda: &mut Vec<AgendaItem>) {
    loop {
        let t = ask(String::from("Time"), String::from("Leave blank when done"));
        if t.is_empty() {
            break;
        }
        let s = ask(String::from("Subject"), String::from("Topic of discussion"));
        let p = ask(
            String::from("Presenter"),
            String::from("Who will present (can be left blank)"),
        );
        let item = AgendaItem {
            time: t,
            subject: s,
            presenter: p,
        };
        agenda.push(item);
    }
}

fn get_agenda_from_csv(
    agenda: &mut Vec<AgendaItem>,
    inputfile: String,
) -> Result<(), Box<dyn Error>> {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b',')
        .flexible(true)
        .trim(csv::Trim::All)
        .comment(Some(b'#'))
        .from_path(std::path::Path::new(&inputfile))?;
    for r in reader.deserialize() {
        let item: AgendaItem = r?;
        agenda.push(item);
    }
    Ok(())
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
    let args = Args::parse();

    let mut file = get_outfile(args.output.as_ref());
    let mut agenda: Vec<AgendaItem> = Vec::new();
    let csv_file = args.csv.clone().unwrap_or_else(|| String::from(""));
    if csv_file.is_empty() {
        get_agenda_items_interactively(&mut agenda);
    } else {
        let _ = get_agenda_from_csv(&mut agenda, csv_file);
    };

    write!(&mut file, "{}", STYLE)?;
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
