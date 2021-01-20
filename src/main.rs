/*
@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
@@@@@@@@@@@@@@@@@@@@@B%3=:,''',;*ug@@@@@@@
@@@@@@@@@@@@BqB@@@N5^              '{B@@@@
@@@@@@@@@@@O   ;\'                   -U@@@
@@@@@NAa$e?                           `O@@
@@@g5r;,:,                             ?@@
@@w;;;'`,,                             ;@@
@d!,;,,|r:                           ,vd@@
@@C;_`,/:-                       ;   `B@@@
@@dc^,`,;+-              '|$N    -:;zjB@@@
@@@Bo;:_:'          `1C=l^  E       !vj@@@
@@@@@D3='           ?;      %N     :Oe^@@@
@@@@@@N,          `+?      ^B@o    !gv*@@@
@@@@@@|             ^    ;@6n@a     _ 7@@@
@@@@@j                    :dBu         -B@
@@@@D.                                 L@@
@@@@@v                               _q@@@
@@@@@@w'                        `,+wB@@@@@
@@@@@@@Bj:               /@@@@@@@@@@@@@@@@
@@@@@@@@@@BK:             iB@@@@@@@@@@@@@@
@@@@@@@@@@@@u           '- :U@@@@@@@@@@@@@

          smol rights reserved
*/


extern crate base64;
extern crate colorful;

use colorful::Colorful;
use colorful::RGB;
use rand::prelude::*;
use base64::{encode};
use std::io;
use std::io::prelude::*;

fn main() {
    let rainbowtime: bool = true;
    // set above to false to disable rainbows (good for logging purposes.)

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let content = &line.unwrap().clone();
        encoder(&content, rainbowtime);
        if content.is_empty() == true {
            break;
        }
    }
}

fn encoder(s: &str, rainbows: bool) {
    for char in encode(s).chars() {
        let mut rng = thread_rng();
        if rainbows == true {        
            print!("{}", &char.to_string().color(RGB::new(rng.gen_range(100..=255), rng.gen_range(100..=255), rng.gen_range(100..=255))));
        } else {
            print!("{}", &char.to_string());
        }
    }
    print!("\n")
}