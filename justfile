watch:
  RUST_LOG=trace RUST_BACKTRACE=1 cargo watch --exec run

pass:
  RUST_LOG=trace RUST_BACKTRACE=1 BITCOIND=$PWD/bin/bitcoind.homebrew cargo run

fail:
  RUST_LOG=trace RUST_BACKTRACE=1 BITCOIND=$PWD/bin/bitcoind.7d3817b29 cargo run

system:
  RUST_LOG=trace RUST_BACKTRACE=1 BITCOIND=bitcoind cargo run
