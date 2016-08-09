#![crate_name = "uu_pwd"]

/*
 * This file is part of the uutils coreutils package.
 *
 * (c) Derek Chiang <derekchiang93@gmail.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

extern crate libc;

#[macro_use]
extern crate uucore;

use std::io::Write;
use std::env;

static NAME: &'static str = "pwd";
static VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub fn uumain(args: Vec<String>) -> i32 {
    let mut opts = uucore::coreopts::CoreOptions();

        .optflag("", "help", "display this help and exit")
        .optflag("", "version", "output version information and exit")

        .parse(args);
        }
    };

    if matches.opt_present("help") {
        let msg = format!("{0} {1}

Usage:
  {0} [OPTION]...

Print the full filename of the current working directory.", NAME, VERSION);
        print!("{}", opts.usage(&msg));
    } else if matches.opt_present("version") {
        println!("{} {}", NAME, VERSION);
    } else {
        println!("{}", env::current_dir().unwrap().display());
    }

    0
}
