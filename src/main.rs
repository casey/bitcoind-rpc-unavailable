use {
  bitcoincore_rpc::{Client, RpcApi},
  std::{
    env,
    net::TcpListener,
    process::{Child, Command},
    thread,
    time::{Duration, Instant},
  },
  tempfile::TempDir,
};

struct Kill(Child);

impl Drop for Kill {
  fn drop(&mut self) {
    self.0.kill().unwrap();
  }
}

const DESCRIPTORS: &str = r#"[
  {"desc":"raw(00145846369f3d6ba366d6f5a903fb5cf4dca3763c0e)#k9wh6v62","timestamp":"now"},
  {"desc":"raw(001420800aabf13f3a4c4ce3ce4c66cecf1d17f21a6e)#6m0hlfh4","timestamp":"now"},
  {"desc":"raw(0014c6bf9715e06d73ebf9b3b02d5cc48d24d8bbabc1)#wyavh36r","timestamp":"now"},
  {"desc":"raw(00141ba7807b3f46af113beaea5c698428ce7138cd8a)#jctdsups","timestamp":"now"},
  {"desc":"raw(00140c1bd27f10fff01b36ddf3c1febaa1acff19b080)#9s6nc3pk","timestamp":"now"},
  {"desc":"raw(00141226e31987e4bc2e63c0ee12908f675e40464b20)#9pp7qm39","timestamp":"now"},
  {"desc":"raw(0014f73f149f7503960a5e849c6ee7a8a8c336f631cb)#qtkxv9fc","timestamp":"now"},
  {"desc":"raw(0014c8ccb4d81ffc769fc5fdd8d7eed69b0e0cae5749)#hn39qayv","timestamp":"now"},
  {"desc":"raw(001498565aead2d67a22a6021d55210f2a917fc22169)#6ar3vwsx","timestamp":"now"},
  {"desc":"raw(001403013248ac0cd9eabe176cad162cda2a19f771e1)#4m47mukd","timestamp":"now"},
  {"desc":"raw(00147de17826fab4e7572755ad8ddbe40ce61f63a699)#03v5x8vf","timestamp":"now"},
  {"desc":"raw(00146402f6560278b5c87cf1173f17fd971e3f0e41a1)#0pjd3l95","timestamp":"now"},
  {"desc":"raw(0014fc7707f8913a5ed05e9c95f79993fa63fc95fb91)#fdxpnn5d","timestamp":"now"},
  {"desc":"raw(00149118bb0f57eea8ba2a8b63c3dd4886ec39b6f559)#v3e0s6wa","timestamp":"now"},
  {"desc":"raw(001424f59b558b0094690d208845d2d4aec7eab05917)#qp3f02gu","timestamp":"now"},
  {"desc":"raw(001407cfbc97a867f3adc6e74a86a12ab50a6c96dfdc)#nacutv73","timestamp":"now"},
  {"desc":"raw(0014ad0e3b494957a6556d48d201dcd08a56a1337e71)#lxs5utkw","timestamp":"now"},
  {"desc":"raw(0014a44766df4820e741cf3e69de06604377803b1d72)#xlduxz49","timestamp":"now"},
  {"desc":"raw(0014ab3ce65379c43bd52a31245b66482ccad72c56e6)#7m30zf3n","timestamp":"now"},
  {"desc":"raw(0014014c4f73107497cbb8a28e95d8558726bcf353b0)#5hn4u62g","timestamp":"now"},
  {"desc":"raw(0014e20f4b7479cd94d63dad0af7d422ab318502edc9)#ankgt3u9","timestamp":"now"},
  {"desc":"raw(0014a4ad33e67d990ce5dbeda8fc033338e878175bff)#kz76eymj","timestamp":"now"},
  {"desc":"raw(00148c59654a57e921c61b36ec55b9f4973809ba3aa7)#4a0gg950","timestamp":"now"},
  {"desc":"raw(00141622125f724ae21931c8ad3fcf6d2251a5ff3101)#pa9v83g6","timestamp":"now"},
  {"desc":"raw(001492888404b62de9dce757beda671f56622979b277)#0w3ev6uv","timestamp":"now"},
  {"desc":"raw(0014e55f7a7a04ccbb3997ac43f9565f7000cc601021)#axc47qx8","timestamp":"now"},
  {"desc":"raw(0014f12cb2e9b420c40ff5e0815b1fdc491f30696e82)#pkp43cmf","timestamp":"now"},
  {"desc":"raw(0014306057cd7518cd37cec725cbb9057c169913dc0e)#0mv0nk0l","timestamp":"now"},
  {"desc":"raw(0014bbbfe144b731a96530b0ce2644482e2e439842bf)#42a244xg","timestamp":"now"},
  {"desc":"raw(0014d142df70072bb78d6422e1f5bcd0144a514d1a30)#7d8xpujn","timestamp":"now"},
  {"desc":"raw(00141fdf3bb05c1b6d780e9054c89bd619609bf1f82d)#y0x67vfq","timestamp":"now"},
  {"desc":"raw(00148939d2d80b6591267c59af6b1614ef28e8505c0d)#w453gc92","timestamp":"now"},
  {"desc":"raw(001429b662406f03efe9c20d9227c463aa0c37613768)#mmdtm3vj","timestamp":"now"},
  {"desc":"raw(0014339611d35e9e2789fd1c2c4c59744cd528c30448)#5cm5cu7x","timestamp":"now"},
  {"desc":"raw(00148db3e38ada7f7da7b853a85564063a34fdbb7bde)#6zvgvvrg","timestamp":"now"},
  {"desc":"raw(001493a50be6af43b13ce318c49a5d4088a446348d86)#fxtearpa","timestamp":"now"},
  {"desc":"raw(0014c325b3bceb1a06526ed375d6af7e21a3b18d3c4f)#x8kut84a","timestamp":"now"},
  {"desc":"raw(0014a86d53c183affc9f41119f9a8a12f1c23ea3f22a)#p307pvnt","timestamp":"now"},
  {"desc":"raw(0014c780b4dd93a2bbdbcda22e1b7e26db56ca4538a0)#juna8rk7","timestamp":"now"},
  {"desc":"raw(0014e4ed854a807959a7ee787e0f21fd749b23cf7971)#4es3xghl","timestamp":"now"},
  {"desc":"raw(0014b75df751f7407cb7adbd281d9f34a34dd5f26b3a)#8hm2usj7","timestamp":"now"},
  {"desc":"raw(0014d5ebb3be03f2ad8810fd4e246a60c1a437f27962)#rymmus3n","timestamp":"now"},
  {"desc":"raw(001411bbabfb5e10d17e6d29fe6870ac48b9ca560234)#6k8wxlzf","timestamp":"now"},
  {"desc":"raw(0014cd701716b718e1cbd474f2f7685c4c7939b18359)#v9rhxfjx","timestamp":"now"},
  {"desc":"raw(0014ebf9b6888c54d99b5df846749e397dee0c18f54d)#c79pex2s","timestamp":"now"},
  {"desc":"raw(0014d7bea15cd50f69ea154fc906004d1b22eda94eec)#0nj2uyj4","timestamp":"now"},
  {"desc":"raw(001489102bee375f6579cbc781267dfe499d0015e85d)#hputch5y","timestamp":"now"},
  {"desc":"raw(0014b79ce1bf47da6786ac1e4abbb53a19a691e2d9bf)#x30zcf0g","timestamp":"now"},
  {"desc":"raw(00144747c2e3c4a4298919490b62c483c5ae044e54d9)#wme4nj20","timestamp":"now"},
  {"desc":"raw(001465bdadb594af71cc17cbab558f9795d12c7cab7f)#nkz9msgr","timestamp":"now"},
  {"desc":"raw(001405aa550fd71c6c78144464106e0e2c6698c1da63)#8462trry","timestamp":"now"},
  {"desc":"raw(00145d67a29230c89cc8312afbe06eedb4252d043ef4)#yqevjyff","timestamp":"now"},
  {"desc":"raw(00140c7eb4ec81793be62515d8683c8ddb6b16151e8e)#pj934vfx","timestamp":"now"},
  {"desc":"raw(0014d7f23a11d8539844fc8478e82f19e91f5ccae342)#8w3r73pj","timestamp":"now"},
  {"desc":"raw(0014360c671c07af4527a56d83b06648dfab0975a13f)#j6darl3v","timestamp":"now"},
  {"desc":"raw(00145d61e5900db40a49346d809e8f4335d68705fff5)#wl06dsvk","timestamp":"now"},
  {"desc":"raw(00144b2a163a76f392d975efef958444f6aaac9ddec8)#3rt3rl27","timestamp":"now"},
  {"desc":"raw(00147beeca8b2390e7c708d3ba83e05d7e34fc055a8e)#z2uxn97h","timestamp":"now"},
  {"desc":"raw(001408a53ef1e2a9ddf2f7b7a4eb574d2b390a2b1c99)#qrjrpl6u","timestamp":"now"},
  {"desc":"raw(0014d55cdd2b295c2e3387838fd8b025923edc88ee63)#crf74usx","timestamp":"now"},
  {"desc":"raw(0014a38c98d4ba62bb9df9db62a1a8bb58eb2a07a07b)#e2pmfkqm","timestamp":"now"},
  {"desc":"raw(0014dd90129a2ed473d5c96f2eb58dca81c01369542e)#y6hsstz4","timestamp":"now"},
  {"desc":"raw(0014af7e5643830997210c65f4370eaf5a7dc764d45e)#m7r3gq88","timestamp":"now"},
  {"desc":"raw(00149cdf1765b03e1cf5145945394aeddc1a0e6693ac)#mhxdece7","timestamp":"now"},
  {"desc":"raw(00148d2e1bac1c70c98e60c34a0e7cea3bca4653f304)#qwpq5fur","timestamp":"now"},
  {"desc":"raw(001459755bf4b210e00fa403a0c556e11b842310abde)#4pyt2tu8","timestamp":"now"},
  {"desc":"raw(0014b449a14ad2951df4723a5db67fa3ed6da564c09b)#m0v3vj8d","timestamp":"now"},
  {"desc":"raw(0014317ddfe4d602826759215e1abea701b9903e486f)#4k8w7762","timestamp":"now"},
  {"desc":"raw(00146cc862530d795fe077586bdbb33f511ac7cb01ff)#8eprpvnv","timestamp":"now"},
  {"desc":"raw(0014ae91241bb67aba596d1ca6cf6e715b73668625be)#rnhyfshy","timestamp":"now"},
  {"desc":"raw(00146c9fa5cc60fc595d839521f43f17a4c6360e2de2)#434dv46f","timestamp":"now"},
  {"desc":"raw(0014b089ceabb72af3a98ef4a05f6c456e9f24c671b2)#d4lcthch","timestamp":"now"},
  {"desc":"raw(0014771ad9bdde222dd69b4941c648409fbe346aa313)#e4fs2a7y","timestamp":"now"},
  {"desc":"raw(0014a8498b2097f007f2ab78c6921041b6d8b8f330de)#dd7pyhz6","timestamp":"now"},
  {"desc":"raw(00146faf2f0528e266cb1d06b356cc031d455bd3dc63)#9uxar8u3","timestamp":"now"},
  {"desc":"raw(0014f9907afae2c3f8be3bce12dca19be2867245f88d)#7y6gnm25","timestamp":"now"},
  {"desc":"raw(001431aacee7ab7794a8766723536c62f05b7f63fc84)#0cf3ks45","timestamp":"now"},
  {"desc":"raw(0014c06f7953519dbfd1e25ff18d001be590401ecba6)#lrnv7qcq","timestamp":"now"},
  {"desc":"raw(0014434089931c3b52d2def67f62eac99d23058a9f09)#cg0nslav","timestamp":"now"},
  {"desc":"raw(0014bf4d1f68349dd65990d6338ac61ca946e98f8cc5)#tx8gf6l3","timestamp":"now"},
  {"desc":"raw(001417a66b36201d81e59f48939c67755358164dec66)#4x3h7lv2","timestamp":"now"},
  {"desc":"raw(0014064bcbe8d6208bb58e2147a5bbcd97dbdcd298a4)#qd9yj6zk","timestamp":"now"},
  {"desc":"raw(0014f2dc57444e57098f00c57060f97da9cc8fadac3c)#f5uwe54v","timestamp":"now"},
  {"desc":"raw(00142fb4648a340c06a2f2f9742d2e7832e8f347a2a2)#q6fx2u8j","timestamp":"now"},
  {"desc":"raw(0014564a50bddba40328fd6bea63ea434f6e47269333)#reh8v5v0","timestamp":"now"},
  {"desc":"raw(001401f256832bf51c8690c921412ee0d89b8dcc5d8d)#l7tyefep","timestamp":"now"},
  {"desc":"raw(0014ad3377bab25e7ce5de2d243c2af1a1a744c5ba25)#n8hylkg4","timestamp":"now"},
  {"desc":"raw(00146cde66581cc43d654da4d139660fce31dbd27821)#58pr9vpr","timestamp":"now"},
  {"desc":"raw(0014d39f838ee2bd8734fb90354d2b15a1c1eb8900ac)#s526lyam","timestamp":"now"},
  {"desc":"raw(001444fafb985f591ccfe2b9990e0e75783bb0bde701)#cpmq7gwp","timestamp":"now"},
  {"desc":"raw(00145d75454b0038bc70f12306484544038558140938)#90h7e468","timestamp":"now"},
  {"desc":"raw(00147a01fddfdee428515cf4c0ac5ac609fd0724620f)#9cejp0ux","timestamp":"now"},
  {"desc":"raw(001406eb1337f2a3daf860b7ac8b9b46c10d0f70010e)#mk362vyg","timestamp":"now"},
  {"desc":"raw(0014d6622202a9ddbd298e0abdaebcdd1ebf13f43dea)#8j4fmku8","timestamp":"now"},
  {"desc":"raw(00148b4e1f7cb1819dd6f3386d3bd3b49f82ec69bbd5)#gap3ycw7","timestamp":"now"},
  {"desc":"raw(001462b2fe372c6ca2a39bb453b17bae10310bcd02bb)#ndftc243","timestamp":"now"},
  {"desc":"raw(0014b9abb44d5504daff04903f5dcfdbd6bb1613d586)#fu2d8tt2","timestamp":"now"},
  {"desc":"raw(0014762aa4cd9edc41765dff328b3662ac7c424ac771)#mh2d5gj8","timestamp":"now"},
  {"desc":"raw(00144426d291d198ccd0df68521758afe8cff08743a8)#c40nwl9x","timestamp":"now"},
  {"desc":"raw(00140b970dd18df6eaaa790b0050b6929587c47f58ea)#wndmanqc","timestamp":"now"}
]"#;

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

  let output = Command::new("bitcoin-cli")
    .arg(format!("-rpccookiefile={}", cookie_file.display()))
    .arg(format!("-rpcport={port}"))
    .arg("createwallet")
    .arg("main")
    .arg("true")
    .output()
    .unwrap();

  assert!(output.status.success(), "{:?}", output);

  let start = Instant::now();

  let output = Command::new("bitcoin-cli")
    .arg(format!("-rpccookiefile={}", cookie_file.display()))
    .arg(format!("-rpcport={port}"))
    .arg("importdescriptors")
    .arg(DESCRIPTORS)
    .output()
    .unwrap();

  assert!(output.status.success(), "{:?}", output);

  dbg!(Instant::now() - start);

  dbg!(output);

  drop(child);
}
