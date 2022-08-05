use {
  bdk::{
    blockchain::{ConfigurableBlockchain, RpcBlockchain, RpcConfig},
    database::MemoryDatabase,
    keys::bip39::Mnemonic,
    template::Bip84,
    KeychainKind, SyncOptions, Wallet,
  },
  bitcoin::Network,
  bitcoincore_rpc::{Client, RpcApi},
  std::{
    env,
    net::TcpListener,
    process::{Child, Command},
    thread,
    time::Duration,
  },
  tempfile::TempDir,
};

struct Kill(Child);

impl Drop for Kill {
  fn drop(&mut self) {
    self.0.kill().unwrap();
  }
}

fn main() {
  env_logger::init();

  let port = TcpListener::bind("127.0.0.1:0")
    .unwrap()
    .local_addr()
    .unwrap()
    .port();

  let tempdir = TempDir::new().unwrap();

  let bin = env::var("BITCOIND").unwrap();

  let child = Kill(
    Command::new(bin)
      .args(&[
        "-regtest",
        "-txindex=1",
        &format!("-datadir={}", tempdir.path().display()),
        &format!("-rpcport={port}"),
      ])
      .spawn()
      .unwrap(),
  );

  let cookie_file = tempdir.path().join("regtest/.cookie");

  while !cookie_file.exists() {
    eprintln!("Waiting for cookie file…");
    thread::sleep(Duration::from_millis(100));
  }

  let client = Client::new(
    &format!("localhost:{port}"),
    bitcoincore_rpc::Auth::CookieFile(cookie_file.clone()),
  )
  .unwrap();

  for attempt in 0..=300 {
    match client.get_blockchain_info() {
      Ok(_) => break,
      Err(err) => {
        if attempt == 300 {
          panic!("Failed to connect to bitcoind: {err}");
        }
      }
    }
    thread::sleep(Duration::from_millis(100));
  }

  let wallet = Wallet::new(
    Bip84(
      (
        Mnemonic::parse("book fit fly ketchup also elevator scout mind edit fatal where rookie")
          .unwrap(),
        None,
      ),
      KeychainKind::External,
    ),
    None,
    Network::Regtest,
    MemoryDatabase::new(),
  )
  .unwrap();

  let blockchain = RpcBlockchain::from_config(&RpcConfig {
    url: format!("localhost:{port}"),
    auth: bdk::blockchain::rpc::Auth::Cookie { file: cookie_file },
    network: Network::Regtest,
    wallet_name: "test".to_string(),
    skip_blocks: None,
  })
  .unwrap();

  wallet.sync(&blockchain, SyncOptions::default()).unwrap();

  drop(child);
}
