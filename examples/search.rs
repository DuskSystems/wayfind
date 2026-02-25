use core::hint::black_box;

use wayfind::{Match, Router};

// cargo asm --package wayfind --example search --simplify --rust search::search
#[inline(never)]
#[must_use]
pub fn search<'r, 'p>(router: &'r Router<usize>, path: &'p str) -> Option<Match<'r, 'p, usize>> {
    router.search(path)
}

fn main() {
    let router: Router<usize> = Router::new();
    black_box(search(black_box(&router), black_box("/test")));
}
