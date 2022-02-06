use std::collections::HashMap;
use std::fs;

fn main() {
    // Skip method can be used with args to avoid the first argument
    // that is usually the command to run the program

    let mut args = std::env::args().skip(1);

    // next() always get the next not listed character from args and
    // unwrap help get the optional value and if is not passed it throw an "panic"
    // and crash the app

    // expect allow us to provide a message if Optional value is not provided

    // let key = args.next().unwrap();
    let key = args.next().expect("Key not provided");

    // let value = args.next().unwrap();

    let value = args.next().expect("Value not provided");

    println!("The key is {} and the value is {}", key, value);

    let contents = format!("{}\t{}\n", key, value);

    let write_result = fs::write("jcs.db", contents);

    match write_result {
        Ok(x) => {
            println!("Saved");
        }
        Err(e) => {
            println!("{}", e);
        }
    }

    let database = Database::new().expect("Something went wrong.");
}

struct Database {
    map: HashMap<String, String>,
}

impl Database {
    fn new() -> Result<Database, std::io::Error> {
        let contents = match fs::read_to_string("jcs.db") {
            Ok(data) => data,
            Err(error) => {
                return Result::Err(error);
            }
        };

        let mut map = HashMap::new();

        for line in contents.lines() {
            // let (key, value) = line.split_once('\t').expect("Corrupt DB");
            let mut vars = line.splitn(2, '\t');
            let key = vars.next().expect("No Key");
            let value = vars.next().expect("No value");
            map.insert(key.to_owned(), value.to_owned());
        }

        return Ok(Database { map: map });
    }
}
