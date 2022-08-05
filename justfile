export RUST_LOG := 'trace'
export RUST_BACKTRACE := '1'

pass:
  BITCOIND=$PWD/bin/bitcoind.homebrew cargo run

fail:
  BITCOIND=$PWD/bin/bitcoind.7d3817b29 cargo run

system:
  BITCOIND=bitcoind cargo run
