//! Read agenda items from user prompts
//! and generate an html table
//! formatted per our standards

#[derive(Debug)]
struct AgendaItem {
    time: String,
    subject: String,
    presenter: String,
}

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

fn get_agenda_item(agenda: &mut Vec<AgendaItem>) {
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

fn main() {
    println!("Table maker");
    let mut agenda: Vec<AgendaItem> = Vec::new();
    get_agenda_item(&mut agenda);
    println!("Agenda Items:");
    println!("-------------");
    println!("{:?}", agenda);
    for i in agenda {
        println!("{}, {}, {}", i.time, i.subject, i.presenter)
    }
}
