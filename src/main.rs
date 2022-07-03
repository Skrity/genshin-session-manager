use std::env;
use std::process;
use registry::{Hive, Security};

fn main() {
    let args: Vec<String> = env::args().collect();
    // if args.len() < 2 {
    //     println!("No command given. Exiting...");
    //     process::exit(1);
    // }
	let session = read_session();
	//println!("The current session is: {:?}", session);
    match args.len() {
        // 2=>match args[1].as_str() {
            // "current"=>println!("One"),
            // _=>{    println!("Expected 1-2 arguments, received {}",args.len()-1);
                    // process::exit(1); },
        // },
        3=>match args[1].as_str() {
            "current"=>current(session),
            "save"=>save(&args[2],session),
            "delete"=>delete(&args[2]),
            "run"=>println!("run"),
            "load"=>load(&args[2]),
            _=>println!("Command not recognized. Exiting..."),
        },
        1|_=>{  println!("Expected 1-2 arguments, received {}",args.len()-1);
                process::exit(1); },
    }
}
    

fn read_session() -> registry::Data {
	
    if let Ok(reg) = Hive::CurrentUser.open(r"SOFTWARE\miHoYo\Genshin Impact", Security::Read) {
		if let Ok(session) = reg.value("MIHOYOSDK_ADL_PROD_OVERSEA_h1158948810") {
			session
		} else {
			println!("No session found. Try logging in to Genshin Impact. Terminating...");
			process::exit(1);
		}
    } else {
        println!("No session found. Try logging in to Genshin Impact. Terminating...");
        process::exit(1);
    }
}


fn read_session_from_store(name : &String) -> registry::Data {
	
    if let Ok(reg) = Hive::CurrentUser.open(r"SOFTWARE\miHoYo\Genshin Impact\sessions", Security::Read) {
		if let Ok(session) = reg.value(name) {
			session
		} else {
			println!("No session found in store. Terminating...");
			process::exit(1);
		}
    } else {
        println!("No session found in store. Terminating...");
        process::exit(1);
    }
}


fn check_sessionstore() {
    if let Err(_) = Hive::CurrentUser.open(r"SOFTWARE\miHoYo\Genshin Impact\sessions", Security::Read) {
        if let Err(_) = Hive::CurrentUser.create(r"SOFTWARE\miHoYo\Genshin Impact\sessions", Security::Read) {
			println!("Couldn't create sessions key. Terminating...");
		}
    }
}


fn save(name : &String, session_data : registry::Data) {
	check_sessionstore();
    println!("saving {}...",name);
	if let Ok(reg) = Hive::CurrentUser.open(r"SOFTWARE\miHoYo\Genshin Impact\sessions", Security::Write) {
		println!("Result: {:?}",reg.set_value(name, &session_data));
	}
}

fn load(name : &String) {
	check_sessionstore();
	let session_data = read_session_from_store(name);
    println!("loading {}...",name);
	if let Ok(reg) = Hive::CurrentUser.open(r"SOFTWARE\miHoYo\Genshin Impact", Security::Write) {
		println!("Result: {:?}",reg.set_value("MIHOYOSDK_ADL_PROD_OVERSEA_h1158948810", &session_data));
	}
	
}

fn delete(name : &String) {
	check_sessionstore();
	let session_data = read_session_from_store(name);
    println!("deleting {}...",name);
	if let Ok(reg) = Hive::CurrentUser.open(r"SOFTWARE\miHoYo\Genshin Impact\sessions", Security::Write) {
		println!("Result: {:?}",reg.delete_value(name));
	}
	
}

fn current(session_data : registry::Data) {
    println!("currenting...");
	if let Ok(reg) = Hive::CurrentUser.open(r"SOFTWARE\miHoYo\Genshin Impact\sessions", Security::Read) {
		for (val, key) in reg.values().zip(reg.keys()) {
				println!("Key: {:?},", key);
			if let Ok(val) = session_data {
				println!("Current session is in store under the name: {:?}", key);
			}
		}
	}
	
}
