// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/io
//
// This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_use]
extern crate io;

fn main() {
    if let Some(c) = read_character!() {
        writeln_character!(&c);
    }
}
