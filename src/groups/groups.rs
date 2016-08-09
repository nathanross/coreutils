#![crate_name = "uu_groups"]

/*
 * This file is part of the uutils coreutils package.
 *
 * (c) Alan Andrade <alan.andradec@gmail.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 *
 */


#[macro_use]
extern crate uucore;

use std::io::Write;
use uucore::c_types::{get_pw_from_args, group};

static NAME: &'static str = "groups";
static VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub fn uumain(args: Vec<String>) -> i32 {
    let matches = uucore::coreopts::CoreOptions();
        .optflag("h", "help", "display this help menu and exit")
        .optflag("V", "version", "display version information and exit")

        .parse(args);
            return 1;
        }
    };

    if matches.opt_present("version") {
        println!("{} {}", NAME, VERSION);
    } else if matches.opt_present("help") {
        let msg = format!("{0} {1}

Usage:
  {0} [OPTION]... [USER]...

Prints the groups a user is in to standard output.", NAME, VERSION);

        print!("{}", opts.usage(&msg));
    } else {
        group(get_pw_from_args(&matches.free), true);
    }

    0
}
