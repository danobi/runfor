use std::env;
use std::process;
use std::thread;
use std::time;

// struct to hold the global variables
struct VarStruct {
    run_time: u64,
    program: String,
    program_args: Vec<String>,
}

// verifies we have enough cmd line args
fn check_cmd_args() -> bool {
    let num_args = env::args().count();
    if num_args < 3 {
        println!("Usage: ./runfor duration program [program-args]");
        return false;
    }
    return true;
}

// loads values into VarStruct and returns an instance
fn load_vars() -> Option<VarStruct> {
    // make sure we have the right # of cmd line args
    if check_cmd_args() == true {
        let num_args = env::args().count();
        let run_time: u64 = env::args()
                                .nth(1)
                                .unwrap()
                                .parse()
                                .ok()
                                .expect("Required a number");
        let program = env::args().nth(2).unwrap();       // program we want to run
        let mut program_args = Vec::new();               // arguments for program
        for i in 3..num_args {
            program_args.push(env::args().nth(i).unwrap());
        }

        return Some(VarStruct {
            run_time: run_time,
            program: program,
            program_args: program_args,
        });
    } else {
        return None;
    }
}

// prints out informative (verbose) message
fn output_vars(vars: &VarStruct) {
    println!("========");
    println!("run time: {}", vars.run_time);
    println!("program: {}", vars.program);
    print!("program args: ");
    for prog_arg in &vars.program_args[..] {
        print!("{} ", prog_arg);
    }
    println!("");
    println!("========\n");
}


fn main() {
    let res = load_vars();
    let vars: VarStruct;
    match res {
        None => return,
        Some(x) => vars = x,
    }

    output_vars(&vars);

    // setup and spawn child
    let mut child = process::Command::new(vars.program)
                        .args(&vars.program_args[..])
                        .spawn()
                        .unwrap_or_else(|e| panic!("failed to execute child: {}", e));

    // now sleep the right amount of time
    thread::sleep(time::Duration::new(vars.run_time, 0));

    // kill the child
    let ecode = child.kill().unwrap_or_else(|e| panic!("couldn't kill child: {}", e));
    assert!(ecode == ());
}
