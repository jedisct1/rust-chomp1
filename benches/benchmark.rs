mod combinators;
mod float;
mod http_bench;

fn main() {
    combinators::bench();
    float::bench();
    http_bench::bench();
}
