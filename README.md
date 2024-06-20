NopNV

Small rust library to parse `.env` files

Usage:

`cargo.toml`

```
[dependencies]
nopnv = { git = "https://github.com/NopFault/nopnv", branch = "master"
```

`main.rs`

```
use nopnv::Env;

fn main() {
    let env = Env::from_file(".env").expect("Failed to read .env file");

    if let Some(hostname) = env.get("HOSTNAME") {
        println!("APP: {}", hostname);
    } else {
        println!("APP not found");
    }

    if let Some(ps) = env.get_array("PS") {
        println!("PS: {:?}", ps);
    }

    if let Some(arr) = env.get_array("TEST_ARR") {
        println!("TST_ARR: {:?}", arr);
    }
}
```

`.env`

```
HOST=localhost
PORT=2233
HOSTNAME=$HOST:$PORT

P1=asd123
P2=dsa321
P3=432qwe

PS=$P1,$P2,$P3
TEST_ARR=asd,1,444
```
