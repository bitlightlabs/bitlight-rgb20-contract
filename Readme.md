# RGB20 Demo based on RGB v0.11

## Pre-request

To complete the demo, you need to set up the following toolchains:

1. [Git][git]
2. [Rust][rust]
3. [RGB-CLI][rgb-cli]
4. [Bitcoin Local Env][bitlight-local-env-public]

[git]: https://git-scm.com/

[rust]: https://www.rust-lang.org/tools/install

[rgb-cli]: https://github.com/RGB-WG/rgb

[bitlight-local-env-public]: https://github.com/bitlightlabs/bitlight-local-env-public/

### RGB-CLI

First, clone it from GitHub:

```bash
git clone https://github.com/RGB-WG/rgb.git
```

Then, build it, copy the binary to `~/.cargo/bin/`:

```
cd rgb
cargo build --package rgb-wallet --bin rgb --release 
cp target/release/rgb ~/.cargo/bin/
```

### Bitcoin Local Env

Run `make up` to start the development environment

```bash
git clone https://github.com/bitlightlabs/bitlight-local-env-public/
make up
```

Write down Alice's xprv and Bitcoin address.

```
make alice-cli
```

```text
Network             :  regtest
Wallet Name         :  alice
Root XPRV           :  tprv8ZgxMBicQKsPeHzjP5LTL818LxwHbJNLZRa98Qdnn7M98fW15365cB1Sz9QZvYufASRKH6JEPhfpxVuFTKMHxDcEAVboqKuZdMmxzKVhMnW
Fixed XPUB          :  [5183a8d8/86'/1'/0']tpubDDtdVYn7LWnWNUXADgoLGu48aLH4dZ17hYfRfV9rjB7QQK3BrphnrSV6pGAeyfyiAM7DmXPJgRzGoBdwWvRoFdJoMVpWfmM9FCk8ojVhbMS/<0;1;9;10>/*
Bitcoin Address     :  bcrt1pn0s2pajhsw38fnpgcj79w3kr3c0r89y3xyekjt8qaudje70g4shs20nwfx
```

Write down Bob's xprv and Bitcoin address.

```
make bob-cli
```

```text
Network             :  regtest
Wallet Name         :  bob
Root XPRV           :  tprv8ZgxMBicQKsPeEP6QyHbs7W2pfW5FJisXLcX93h2AnH5Kx8fuhKz7FYm4kw46SUgXJd3zUKwNoTqxtpLw7vmtLeFUGJb6XSeom45hQjeXxJ
Fixed XPUB          :  [3abb3cbb/86'/1'/0']tpubDDeBXqUqyVcbe75SbHAPNXMojntFu5C8KTUEhnyQc74Bty6h8XxqmCavPBMd1fqQQAAYDdp6MAAcN4e2eJQFH3v4txc89s8wvHg98QSUrdL/<0;1;9;10>/*
Bitcoin Address     :  bcrt1plphl407vyfpml2thhypzuqk232256njnaw4zhtmyrrku66pqn9usx4x59h
```

get satoshis faucet for alice and bob

```
make core-cli
load_wallet
send bcrt1pn0s2pajhsw38fnpgcj79w3kr3c0r89y3xyekjt8qaudje70g4shs20nwfx 1
send bcrt1plphl407vyfpml2thhypzuqk232256njnaw4zhtmyrrku66pqn9usx4x59h 1
mint 1 
```

## Create and Import Contract

To create a RGB20 contract, just clone this repo to you local machine. Then
compile and run it.

```bash
git clone https://github.com/bitlightlabs/bitlight-rgb20-contract
cd bitlight-rgb20-contract
```

edit main.rs, change the beneficiary to alice's address

```rust
    let beneficiary_txid =
        Txid::from_hex("d6afd1233f2c3a7228ae2f07d64b2091db0d66f2e8ef169cf01217617f51b8fb").unwrap();
    let beneficiary = Outpoint::new(beneficiary_txid, 1);
```

```
make run
```

```text
The issued contract data:
{"naming":{"ticker":"TEST","name":"Test asset","details":null},"precision":"centiMicro"}
amount=100000000000, owner=bc:tapret1st:d6afd1233f2c3a7228ae2f07d64b2091db0d66f2e8ef169cf01217617f51b8fb:1, witness=~
totalSupply=100000000000

Contracts are available in the examples directory
---------------------------------
./examples:
-rw-r--r--  1 bitlight  staff  21364 Mar 15 19:57 rgb20-simplest.rgb
-rw-r--r--  1 bitlight  staff  27456 Mar 15 19:57 rgb20-simplest.rgba
---------------------------------
```

Now, we are creating a RGB20 #TEST contract, which stores in `examples` fold.

Before importing contracts, let's import our wallets to rgb.

Export ESPLORA_SERVER env for esplora-api endpoint:

```bash
export LNPBP_NETWORK=regtest
export ESPLORA_SERVER="http://esplora-api.bitlight-local-env.orb.local:3000"
```

Create rgb wallet container for Alice:

```bash
$ rgb-d .alice create default --tapret-key-only <alice-fixed-xpub-descriptor>
$ rgb-d .alice create default --tapret-key-only "[5183a8d8/86'/1'/0']tpubDDtdVYn7LWnWNUXADgoLGu48aLH4dZ17hYfRfV9rjB7QQK3BrphnrSV6pGAeyfyiAM7DmXPJgRzGoBdwWvRoFdJoMVpWfmM9FCk8ojVhbMS/<0;1;9;10>/*"

Loading descriptor from command-line argument ... success
Syncing keychain 0 ........... keychain 1 .......... keychain 9 .......... keychain 10 .......... success
Saving the wallet as 'default' ... success

$ rgb -d .alice utxos

Height	   Amount, ṩ	Outpoint
bcrt1pn0s2pajhsw38fnpgcj79w3kr3c0r89y3xyekjt8qaudje70g4shs20nwfx	&0/0
102	   100000000	d6afd1233f2c3a7228ae2f07d64b2091db0d66f2e8ef169cf01217617f51b8fb:1

Loading descriptor from wallet default ... success

Wallet total balance: 100000000 ṩ

```

Create rgb wallet container for Bob:

```bash
$ rgb-d .bob create default --tapret-key-only <bob-fixed-xpub-descriptor>

Loading descriptor from command-line argument ... success
Syncing keychain 0 ........... keychain 1 .......... keychain 9 .......... keychain 10 .......... success
Saving the wallet as 'default' ... success

$ rgb -d .bob utxos

Height	   Amount, ṩ	Outpoint
bcrt1plphl407vyfpml2thhypzuqk232256njnaw4zhtmyrrku66pqn9usx4x59h	&0/0
102	   100000000	389bf4b8c31d5dca7e36cffc361a8a020916cb242015e816ccacfa5312be564c:0

Loading descriptor from wallet default ... success

Wallet total balance: 100000000 ṩ

```

Import contract for Alice

```
$ rgb -d .alice import examples/rgb20-simplest.rgb

Loading descriptor from wallet default ... success
Loading stock ... stock file is absent, creating a new one ... success
Contract urn:lnp-bp:consignment:54zd2k-aPk6BV2u-SKa6HDPh-tCp86dPm-h4N6Ebo8-AQt1Pw#quest-hostel-pulse imported to the stash
```

After that, we can inspect contracts state with `rgb state` command.

get the state of the contract for Alice

```bash
$ rgb -d .alice contracts
rgb:2bLwMXo-deVgzKq97-GUVy6wXea-G1nE84nxw-v5CX3WSJN-mbhsMn7

# rgb -d <DATA_DIR> state <CONTRACT_ID> <IFACE>
$ rgb -d .alice state rgb:2bLwMXo-deVgzKq97-GUVy6wXea-G1nE84nxw-v5CX3WSJN-mbhsMn7 RGB20

Global:
  spec := (naming=(ticker=("TEST"), name=("Test asset"), details=~), precision=8)
  data := (terms=(""), media=~)
  issuedSupply := (100000000000)
  created := (1710580778)

Owned:
  assetOwner:
    amount=100000000000, utxo=bc:tapret1st:d6afd1233f2c3a7228ae2f07d64b2091db0d66f2e8ef169cf01217617f51b8fb:1, witness=~ # owned by the wallet
```

Import contract For Bob:

```bash
$ rgb -d .bob import examples/rgb20-simplest.rgb
$ rgb -d .bob contracts
rgb:2bLwMXo-deVgzKq97-GUVy6wXea-G1nE84nxw-v5CX3WSJN-mbhsMn7

$ rgb -d .bob state rgb:2bLwMXo-deVgzKq97-GUVy6wXea-G1nE84nxw-v5CX3WSJN-mbhsMn7 RGB20

Global:
  spec := (naming=(ticker=("TEST"), name=("Test asset"), details=~), precision=8)
  data := (terms=(""), media=~)
  issuedSupply := (100000000000)
  created := (1710580778)

Owned:
  assetOwner:
```

Now we have successfully created an rgb20 token, and the owner
is `bc:tapret1st:d6afd1233f2c3a7228ae2f07d64b2091db0d66f2e8ef169cf01217617f51b8fb:1`, which belongs to Alice.

## Transfer

There are about five steps in a complete transfer:

1. Create an invoice
2. Construct a PSBT
3. Make a transfer
4. Accept the transfer
5. Sign the PSBT and broadcast it

### Create an invoice

To receive 1,000 #Test, Bob needs to create an invoice and send it to Alice.

```bash
$ rgb -d .bob invoice rgb:2bLwMXo-deVgzKq97-GUVy6wXea-G1nE84nxw-v5CX3WSJN-mbhsMn7 RGB20 1000 --address-based 
rgb:2bLwMXo-deVgzKq97-GUVy6wXea-G1nE84nxw-v5CX3WSJN-mbhsMn7/RGB20/1000+bcrt:p9yjaffzhuh9p7d9gnwfunxssngesk25tz7rudu4v69dl6e7w7qhq5x43k5
```

### Make a transfer

```bash
$ rgb -d .alice transfer \
    rgb:2bLwMXo-deVgzKq97-GUVy6wXea-G1nE84nxw-v5CX3WSJN-mbhsMn7/RGB20/1000+bcrt:p9yjaffzhuh9p7d9gnwfunxssngesk25tz7rudu4v69dl6e7w7qhq5x43k5 \
    transfer.consignment alice.psbt
```

The consignment is saved in the `transfer.consignment` file, and needs to be sent to Bob, who is waiting to accept it.

### Accept transfer

After receiving the `transfer.consignment` file, Bob could validate it before accepting.

```bash
$ rgb -d .bob validate transfer.consignment
The provided consignment is valid
```

Bob accepts the consignment:

```bash
$ rgb -d .bob accept -f transfer.consignment
Transfer accepted into the stash
```

### Sign psbt and broadcast it

#### Sign psbt

Let's inspect the psbt file:

```bash
$ hal psbt decode --regtest alice.psbt
```

```json
{
  "unsigned_tx": {
    "txid": "0fd2b9f690428f751aa381c82cadf52b50ced63616802fe0803cb983d5ac3e1a",
    "wtxid": "0fd2b9f690428f751aa381c82cadf52b50ced63616802fe0803cb983d5ac3e1a",
    "size": 137,
    "weight": 548,
    "vsize": 137,
    "version": 2,
    "locktime": 0,
    "inputs": [
      {
        "prevout": "d6afd1233f2c3a7228ae2f07d64b2091db0d66f2e8ef169cf01217617f51b8fb:1",
        "txid": "d6afd1233f2c3a7228ae2f07d64b2091db0d66f2e8ef169cf01217617f51b8fb",
        "vout": 1,
        "script_sig": {
          "hex": "",
          "asm": ""
        },
        "sequence": 0,
        "witness": null
      }
    ],
    "outputs": [
      {
        "value": 99997600,
        "script_pub_key": {
          "hex": "51209aa1750ffc777b77b7a5e92202e75980072afbec58d51c5130d5857445c8ab20",
          "asm": "OP_PUSHNUM_1 OP_PUSHBYTES_32 9aa1750ffc777b77b7a5e92202e75980072afbec58d51c5130d5857445c8ab20",
          "type": "unknown",
          "address": "bcrt1pn2sh2rluwaah0da9ay3q9e6esqrj47lvtr23c5fs6kzhg3wg4vsqldfds2"
        }
      },
      {
        "value": 2000,
        "script_pub_key": {
          "hex": "51202925d4a457e5ca1f34a89b93c99a109a330b2a8b1787c6f2acd15bfd67cef02e",
          "asm": "OP_PUSHNUM_1 OP_PUSHBYTES_32 2925d4a457e5ca1f34a89b93c99a109a330b2a8b1787c6f2acd15bfd67cef02e",
          "type": "unknown",
          "address": "bcrt1p9yjaffzhuh9p7d9gnwfunxssngesk25tz7rudu4v69dl6e7w7qhq5x43k5"
        }
      }
    ],
    "total_output_value": 99999600
  },
  "inputs": [
    {
      "witness_utxo": {
        "value": 100000000,
        "script_pub_key": {
          "hex": "51209be0a0f65783a274cc28c4bc5746c38e1e3394913133692ce0ef1b2cf9e8ac2f",
          "asm": "OP_PUSHNUM_1 OP_PUSHBYTES_32 9be0a0f65783a274cc28c4bc5746c38e1e3394913133692ce0ef1b2cf9e8ac2f",
          "type": "unknown",
          "address": "bcrt1pn0s2pajhsw38fnpgcj79w3kr3c0r89y3xyekjt8qaudje70g4shs20nwfx"
        }
      }
    }
  ],
  "outputs": [
    {},
    {}
  ]
}
```

Now, it's Alice's turn to sign the PSBT file and broadcast it.

Sign the PSBT with sparrow, hal, or bdk-cli.

After broadcasting, mine 1 block.

```
cargo run --bin broadcast_tx
```

```
cd <bitlight-local-env-public>
make core-cli
mint 1
```

At that time, Bob and Alice would get different outputs with `rgb state` and `rgb validate`.

### Check Alice and Bob's RGB state

For Alice:

```bash
$ RGB_BIT_CONTRACT="rgb:2bLwMXo-deVgzKq97-GUVy6wXea-G1nE84nxw-v5CX3WSJN-mbhsMn7"
$ rgb -d .alice state $RGB_BIT_CONTRACT RGB20 --sync
$ rgb -d .alice state $RGB_BIT_CONTRACT RGB20

Global:
  spec := (naming=(ticker=("TEST"), name=("Test asset"), details=~), precision=8)
  data := (terms=(""), media=~)
  issuedSupply := (100000000000)
  created := (1710580778)

Owned:
  assetOwner:
    amount=99999999000, utxo=bc:tapret1st:0fd2b9f690428f751aa381c82cadf52b50ced63616802fe0803cb983d5ac3e1a:0, witness=bc:0fd2b9f690428f751aa381c82cadf52b50ced63616802fe0803cb983d5ac3e1a # owned by the wallet
```

For Bob:

```bash
rgb -d .bob state $RGB_BIT_CONTRACT RGB20 --sync
rgb -d .bob state $RGB_BIT_CONTRACT RGB20
Global:
  spec := (naming=(ticker=("TEST"), name=("Test asset"), details=~), precision=8)
  data := (terms=(""), media=~)
  issuedSupply := (100000000000)
  created := (1710580778)

Owned:
  assetOwner:
    amount=1000, utxo=bc:tapret1st:0fd2b9f690428f751aa381c82cadf52b50ced63616802fe0803cb983d5ac3e1a:1, witness=bc:0fd2b9f690428f751aa381c82cadf52b50ced63616802fe0803cb983d5ac3e1a # owned by the wallet
```
