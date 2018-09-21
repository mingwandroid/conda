extern crate zstd;

use std::io;

fn uncompress_stdio() {
    // Uncompress input and print the result.
    zstd::stream::copy_decode(io::stdin(), io::stdout()).unwrap();
}
