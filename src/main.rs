//! Read agenda items from user prompts
//! and generate an html table
//! formatted per our standards
use std::error::Error;
use std::fs;
use std::io::Write;

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
    // by default the input string includes the Enter key
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

fn main() -> Result<(), Box<dyn Error>> {
    println!("Table maker");

    let mut file = fs::File::create("src/agenda.html").expect("Creation Failed");
    let mut agenda: Vec<AgendaItem> = Vec::new();
    get_agenda_items_interactively(&mut agenda);
    let style: String = fs::read_to_string("src/style.html")?;

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
