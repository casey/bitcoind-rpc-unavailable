export RUST_LOG = trace
export RUST_BACKTRACE = 1

fast: install-bitcoind-with-homebrew
	BITCOIND=bitcoind cargo run

slow: build-bitcoind-from-source
	BITCOIND=$$PWD/bitcoin/src/bitcoind cargo run

install-bitcoind-with-homebrew:
	brew install bitcoin

build-bitcoind-from-source:
	brew install automake libtool boost pkg-config libevent
	[[ -d bitcoin ]] || git clone https://github.com/bitcoin/bitcoin.git --depth 1
	[[ -f bitcoin/Makefile ]] || (cd bitcoin && ./autogen.sh && ./configure --with-gui=no --without-bdb)
	[[ -f bitcoin/src/bitcoind ]] || (cd bitcoin && make src/bitcoind)
