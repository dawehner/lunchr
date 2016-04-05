extern crate docopt;
extern crate rustc_serialize;
extern crate users;
extern crate glob;
extern crate regex;

use docopt::Docopt;
use std::env;
use std::string::String;
use users::get_current_uid;
use std::vec::Vec;
use glob::glob;
use std::clone::Clone;
use std::process::Command as pCommand;
use regex::Regex;

#[derive(RustcDecodable, Debug)]
enum Command {
  Start,
  Stop,
  Restart,
  Status,
  Ls,
  Install,
  Uninstall,
  Show,
  Edit,
  Help
}

#[derive(Debug, RustcDecodable)]
struct Args {
  arg_name: Option<String>,
  flag_write: bool,
  flag_force: bool,
  arg_command: Option<String>,
  cmd_start: bool,
  cmd_stop: bool,
  cmd_restart: bool,
  cmd_status: bool,
  cmd_ls: bool,
  cmd_install: bool,
  cmd_uninstall: bool,
  cmd_show: bool,
  cmd_edit: bool
}

fn compute_args(argv: Vec<String>) -> Args {
// lunchr <command> <name> [<args>...]
  const USAGE: &'static str = "
  Usage: lunchr start [-w | --write] [-F | --force] <name>
         lunchr stop
         lunchr restart
         lunchr status
         lunchr ls
         lunchr install
         lunchr uninstall
         lunchr show
         lunchr edit
         lunchr (-h | --help)

    Options:
      -h --help     Show help message
      -F --force    Force start/stop/restart
      -w --write    TODO

  ";

  let docopt = Docopt::new(USAGE);

  let args: Args = docopt.and_then(|d| return d.argv(argv).decode()).unwrap_or_else(|e| e.exit());
  return args;
}

fn match_command(args: &Args) -> Command {
  if args.cmd_start {
    Command::Start
  }
  else if args.cmd_stop {
    Command::Stop
  }
  else if args.cmd_restart {
    Command::Restart
  }
  else if args.cmd_status {
    Command::Status
  }
  else if args.cmd_ls {
    Command::Ls
  }
  else if args.cmd_install {
    Command::Install
  }
  else if args.cmd_uninstall {
    Command::Uninstall
  }
  else if args.cmd_show {
    Command::Show
  }
  else if args.cmd_edit {
    Command::Edit
  }
  else {
    Command::Help
  }
}

fn command_start(name: String) {
  println!("{}", name);
  let plists = find_plists();

  find_daemons(name);
//   let command = Command::new("launchctl")
  //                  .arg("load")
}

fn command_ls() {
  for plist in find_plists() {
    println!("{}", plist);
  }
}

fn find_daemons(name: String) -> Vec<String> {
  let plists = find_plists();
  let re: Regex = Regex::new(name.as_ref()).unwrap();
  let mut files: Vec<String> = vec![];

  for plist in plists {
    println!("{}", plist);
    if re.is_match(plist.as_ref()) {
       files.push(plist);
    }
  }

  if files.len() > 1 {
    println!("Multiple daemons found ...");
  }
  else if files.len() == 0 {
    println!("No daemon found");
  }

  return files;
}

fn find_plists() -> Vec<String> {
  let mut plists: Vec<String> = vec![];
  let dirs = plist_dirs();

  for dir in dirs {
    for entry in glob(format!("{}{}", dir, "/*.plist").as_ref()).unwrap() {
      match entry {
        Ok(path) => {
          let path_as_str = path.into_os_string().into_string().unwrap();
          plists.push(path_as_str);
        },
        Err(e) => println!("{:?}", e),
      }
    }
  }

  return plists;
}

fn is_root_process() -> bool {
  return get_current_uid() == 0;
}

fn plist_dirs() -> Vec<String>{
  let mut dirs: Vec<String> = vec!["/Library/LaunchAgents".to_string()];

  match env::home_dir() {
    Some(ref p) => {
      let st:  String = p.into_os_string().into_string().unwrap();
      let st2: String = format!("{}{}", st, "~/Library/LaunchAgents");
      dirs.push(st2);
    }
    None => println!("meh")
  }
  
  if is_root_process() {
    dirs.push("/Library/LaunchDaemons".to_string());
    dirs.push("/System/Library/LaunchDaemons".to_string());
  }
  return dirs;
}

fn main() {
    let args = compute_args(env::args().map(|res| res).collect());

    println!("{:?}", args);
    let command = match_command(&args);

    match command {
      Command::Start => command_start(args.arg_name.unwrap()),
      Command::Ls => command_ls(),
      _ => println!("Muh")
    }
}
