extern mod pcre;

use pcre::pcre::{compile};
use pcre::consts::{PCRE_CASELESS};

fn find_route(url: &str) -> bool {
    let r = compile("^/todos/?", PCRE_CASELESS);
    false
}
