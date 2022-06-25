extern crate home;
use home::home_dir;
use std::env;
use std::process::Command;

fn main() {
	let args: Vec<String> = env::args().collect();
	let home = home_dir().unwrap().into_os_string().into_string().unwrap();
	Command::new("mkdir").arg("-p").arg(home.clone() + "/.config/skel").output().expect("Failed to create config directory");

	match &args[1][..] {	
        // Add a template to the registry
        "--add" | "-a" => {
            assert_eq!(args.len(), 3, "Wrong number of arguments");
		    Command::new("cp").arg("-R").arg(&args[2]).arg(home.clone() + "/.config/skel").output().expect("Failed to copy directory");
		    println!("Added {} to templates", &args[2]);
	    },

        // Remove a template from the registry
        "--remove" | "-r" => {
		    for i in 2..args.len() {
			    Command::new("rm").arg("-rf").arg(home.clone() + "/.config/skel/" + &args[i]).output().expect("Failed to delete template");
			    println!("Removed {} template", &args[i]);
		    }
        },

        // Create a folder using a template
        _ => {
		    assert!(args.len() == 2 || args.len() == 3, "Wrong number of arguments");
		    let mut target = ".";
		    if args.len() == 3 {
			    target = &args[2];
		    }
		    Command::new("cp").arg("-R").arg(home.clone() + "/.config/skel/" + &args[1]).arg(&target).output().expect("Failed to create directory");
		    if target == "." {
			    println!("Created {} from template", &args[1]);
		    } else {
			    println!("Created {} from template '{}'", &args[2], &args[1]);
		    }
	    }
    }
}
