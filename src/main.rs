use std::env;
use registry::{Hive, Security};


const MHY_KEY: &str = r"SOFTWARE\miHoYo\Genshin Impact";
const MHY_REG: &str = "MIHOYOSDK_ADL_PROD_OVERSEA_h1158948810";


const SESSIONS_KEY: &str = r"SOFTWARE\miHoYo\Genshin Impact\sessions";

fn main() {
    let args: Vec<String> = env::args().collect();

    check_sessionstore();

    let (command, name): (&str, Option<&str>);
    match args.len() {
        2 => { { (command, name) = (args[1].as_str(), None); } },
        3 => { (command, name) = (args[1].as_str(), Some(args[2].as_str())); },
        _ => { panic!("Expected 1-2 arguments, received {}", args.len()-1); },
    }
    match command {
        "c"|"current"=>current(),
        "s"|"save"=>save(handle_name(name)),
        "d"|"delete"=>delete(handle_name(name)),
        "l"|"list"=>println!("{:?}", list_sessions(name)),
        "r"|"load"=>load(handle_name(name)),
        _=> panic!("Command not recognized. Exiting..."),
    }
}

fn handle_name(name: Option<&str>) -> &str {
    match name {
        Some(name) => name,
        None => panic!("This command requires an argument"),
    }
}

// Creates Address Tuple, 1 &str is registry key, second is name
fn select_key(name: Option<&str>) -> (&str, &str) {
    match name {
        Some(x) => { (SESSIONS_KEY, x) },
        None => { (MHY_KEY, MHY_REG) },
    }
}

// Takes session name or None, returns Data read
fn read_session(name: Option<&str>) -> registry::Data {
	let addr = select_key(name);

    if let Ok(reg) = Hive::CurrentUser.open(addr.0, Security::Read) {
		if let Ok(session) = reg.value(addr.1) {
			session
		} else {
			panic!("No session found. Try logging in to Genshin Impact. Terminating...");
		}
    } else {
        panic!("Cannot read sessions key. Try logging in to Genshin Impact. Terminating...");
    }
}

fn write_session(data: registry::Data, name: Option<&str>) {
	let addr = select_key(name);
	if let Ok(reg) = Hive::CurrentUser.open(addr.0, Security::Write) {
		println!("Result: {:?}",reg.set_value(addr.1, &data));
	}
}

fn check_sessionstore() {
    if let Err(_) = Hive::CurrentUser.open(SESSIONS_KEY, Security::Read) {
        if let Err(_) = Hive::CurrentUser.create(SESSIONS_KEY, Security::Read) {
			panic!("Couldn't create sessions key. Terminating...");
		}
    }
}

fn save(name : &str) {
    println!("saving {}...",name);
	write_session(read_session(None), Some(name))
}

fn load(name : &str) {
    let list = list_sessions( Some(name) );
    match list.len() {
        0=>println!("Session {} not found.",name),
        1=>{
            println!("Loading session: {}...",list[0]);
            write_session(read_session(Some(&list[0])), None)
        },
        _=>println!("Session {} matches multiple sessions: {:?}", name, list),
    }
}

fn delete(name : &str) {
    println!("deleting {}...",name);
	if let Ok(reg) = Hive::CurrentUser.open(SESSIONS_KEY, Security::Write) {
		println!("Result: {:?}",reg.delete_value(name));
	}
}

fn current() {
    let current_data = &read_session(None);
    for value in list_sessions(None) {
        let session_data = &read_session(Some(value.as_str()));
        if current_data.to_string() == session_data.to_string() {
            println!("Current session is in session store under the name: {}", value);
        }
    }
}

fn list_sessions( name_pattern: Option<&str> ) -> Vec<String> {
    let mut sessions: Vec<String> = vec![];
	if let Ok(reg) = Hive::CurrentUser.open(SESSIONS_KEY, Security::Read) {
		for value in reg.values() {
            let key = value.as_ref().unwrap().name().to_string_lossy();
            match name_pattern {
                Some(name_pattern) => { if key.contains(name_pattern) { sessions.push(key) } ; },
                None => { sessions.push(key); }
            }
		}
    }
    sessions
}
