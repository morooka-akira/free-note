#[derive(Debug)]
enum Event {
    Update,
    Delete,
    Unknown,
}

type Message = String;

fn parse_log(line: &str) -> (Event, Message) {
    let parts: Vec<_> = line.splitn(2, ' ').collect();

    if parts.len() == 1 {
        return (Event::Unknown, String::from(line));
    }

    let event = parts[0];
    let rest = String::from(parts[1]);

    match event {
        "UPDATE" | "update" => (Event::Update, rest),
        "DELETE" | "delete" => (Event::Delete, rest),
        _ => (Event::Unknown, String::from(line)),
    }
}

fn main() {
    let log =
        "BEGIN Transaction XK341 UPDATE 234:LS/32231 {\"price\": 31.00} -> {\"price\": 40.00} \
               DELETE 342:LO/22111";

    for line in log.lines() {
        let (event, message) = parse_log(line);
        println!("{:?}: {}", event, message);
    }
}
