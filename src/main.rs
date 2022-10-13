use std::env;
use registry::{
    Hive::CurrentUser as HKCU,
    Security,
    Data
};


const MHY_KEY: &str = r"SOFTWARE\miHoYo\Genshin Impact";
const MHY_REG: &str = "MIHOYOSDK_ADL_PROD_OVERSEA_h1158948810";
const SESSIONS_KEY: &str = r"SOFTWARE\miHoYo\Genshin Impact\sessions";

fn main() {
    let args: Vec<String> = env::args().collect();

    check_sessionstore();

    let command = if let Some(c) = args.get(1) { c } else {
        println!("No command specified.");
        return
    };
    let name = args.get(2);

    match command.as_str() {
        "current"=>current(),
        "save"=>save(handle_name(name)),
        "delete"=>delete(handle_name(name)),
        "list"=>print_list("Sessions in store", list_sessions(None)),
        "find"=>print_list("Sessions found", list_sessions(name)),
        //"run"=>println!("run"),
        "load"=>{
            let list = list_sessions(name);
            match list.len() {
                0 => println!("Supplied name not in session store."),
                1 => load(&list[0]),
                _ => println!("Too many matches with this name")
            }
        },
        _=> panic!("Command not recognized. Exiting..."),
    }
}

fn handle_name(name: Option<&String>) -> &str {
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
fn read_session(name: Option<&str>) -> Data {
	let addr = select_key(name);

    if let Ok(reg) = HKCU.open(addr.0, Security::Read) {
		if let Ok(session) = reg.value(addr.1) {
			session
		} else {
			panic!("No session found. Try logging in to Genshin Impact. Terminating...");
		}
    } else {
        panic!("Cannot read sessions key. Try logging in to Genshin Impact. Terminating...");
    }
}

fn write_session(data: Data, name: Option<&str>) {
	let addr = select_key(name);
	if let Ok(reg) = HKCU.open(addr.0, Security::Write) {
		println!("Result: {:?}",reg.set_value(addr.1, &data));
	}
}

fn check_sessionstore() {
    if let Err(_) = HKCU.open(SESSIONS_KEY, Security::Read) {
        if let Err(_) = HKCU.create(SESSIONS_KEY, Security::Read) {
			panic!("Couldn't create sessions key. Terminating...");
		}
    }
}

fn save(name : &str) {
    println!("saving {}...",name);
	write_session(read_session(None), Some(name))
}

fn load(name : &str) {
    println!("loading {}...",name);
	write_session(read_session(Some(name)), None)
}

fn delete(name : &str) {
    println!("deleting {}...",name);
	if let Ok(reg) = HKCU.open(SESSIONS_KEY, Security::Write) {
		println!("Result: {:?}",reg.delete_value(name));
	}
}

fn current() {
    let session_data = &read_session(None);
    let mut found_one = false;
	if let Ok(reg) = HKCU.open(SESSIONS_KEY, Security::Read) {
		for value in reg.values() {
            let key = value.as_ref().unwrap().name();
            // This for some reason creates different size of vec
            //let data: Data = value.as_ref().unwrap().data().to_owned();
            let data = &reg.value(key).unwrap();
            if session_data.to_string() == data.to_string() {
				println!("Current session is in session store under the name: {}", key.to_string_lossy());
                found_one = true;
            };
		}
	}
    if !found_one {
        println!("Current session is not in the session store.");
    }
}

fn list_sessions( name_pattern: Option<&String> ) -> Vec<String> {
    let mut sessions: Vec<String> = vec![];
	if let Ok(reg) = HKCU.open(SESSIONS_KEY, Security::Read) {
		for value in reg.values() {
            let key = value.unwrap().name().to_string_lossy();
            let matches = match name_pattern {
                Some(name_pattern) => key.contains(name_pattern),
                None => true,
            };
            if matches { sessions.push(key) }
		}
    }
    sessions
}

fn print_list(msg: &str, list: Vec<String>) {
    print!("{}: ", msg);
    for l in list {
        print!("{} ", l);
    }
    print!("\n");
}

fn _run() {
    use std::process::Command;
    let mut child = Command::new(r"C:\scoop\apps\rufus\current\rufus.exe").spawn().unwrap();
    let _result = child.wait().unwrap();
}