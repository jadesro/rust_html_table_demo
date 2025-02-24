//! Read agenda items from user prompts
//! and generate an html table
//! formatted per our standards
use clap::Parser;
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

fn inquire(prompt: String) -> String {
    let mut line = String::new();
    println!("{}: ", prompt);
    let _b1 = std::io::stdin().read_line(&mut line).unwrap();
    // by default the input string includes the Enter key (newline character)
    // we remove it here with trim() and ensure we return a string
    line.trim().to_string()
}

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

    let path_to_style = Some(String::from("src/style.html"));
    let style = std::fs::read_to_string(
        path_to_style.ok_or_else(|| String::from("Can't read style.html"))?,
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
