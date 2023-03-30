use std::env;
use std::process::Command;

fn main() -> std::io::Result<()> {
    //std::fs::write("/tmp/args", format!("invoked with {:?}", env::args()))?;

    let mut args = Vec::<String>::with_capacity(env::args().len());
    let mut z_start = false;

    // XXX parse output of crle -64 to get standard lib paths instead of hard
    // coding?
    args.push("-L".into());
    args.push("/lib/64".into());
    args.push("-L".into());
    args.push("/usr/lib/64".into());

    // for PT_INTERP
    args.push("-I".into());
    args.push("/usr/lib/amd64/ld.so.1".into());

    // set target explicitly (is this required)?
    args.push("-m".into());
    args.push("elf_x86_64".into());

    for arg in env::args() {
        if arg == env::current_exe()?.into_os_string().into_string().unwrap() {
            continue;
        }

        if arg == "-C" {
            // remove chdir
        } else if arg == "-z" {
            z_start = true;
        } else if z_start {
            if arg == "ignore" {
                // replace "-z ignore" with --as-needed" for mold
                args.push("--as-needed".into());
            } else {
                // otherwise, emit whatever it was
                args.push("-z".into());
                args.push(arg.clone());
            }

            z_start = false;
        } else if arg == "-Wl,-zdefaultextract" {
            // ignore, mold doesn't recognize this arg
        } else {
            // insert arg unmodified
            args.push(arg.clone());
        }
    }

    //std::fs::write("/tmp/args", format!("passing {}", args.join(" ")))?;

    env::set_current_dir(env::current_dir()?)?;

    // run mold
    let mut cmd = Command::new("/home/james/mold/build/mold");

    for arg in args {
        cmd.arg(arg);
    }

    match cmd.output() {
        Ok(output) => {
            println!("{}", String::from_utf8_lossy(&output.stdout));
            eprintln!("{}", String::from_utf8_lossy(&output.stderr));

            assert!(output.status.success());
            Ok(())
        }

        Err(e) => Err(e),
    }
}
