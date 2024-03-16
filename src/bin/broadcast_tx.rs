use bdk::bitcoin::consensus::encode::deserialize;
use bdk::bitcoin::Transaction;
use bdk_esplora::esplora_client;

fn main() {
    let client =
        esplora_client::Builder::new("http://esplora-api.bitlight-local-env.orb.local:3000")
            .build_blocking()
            .unwrap();

    let signed_tx_hex = "02000000000101fbb8517f611712f09c16efe8f2660ddb91204bd6072fae28723a2c3f23d1afd601000000000000000002a0d7f505000000002251209aa1750ffc777b77b7a5e92202e75980072afbec58d51c5130d5857445c8ab20d0070000000000002251202925d4a457e5ca1f34a89b93c99a109a330b2a8b1787c6f2acd15bfd67cef02e0140735ce01d587d7dff3c935c5a0a0c7551a16454ca7eb7eb5093d550aefb0e4ac808156b08f80d1abd5c0494f8dd77f4a278b44d70ef865254b72f4bd775fb120100000000";
    let signed_tx: Transaction = deserialize(&hex::decode(signed_tx_hex).unwrap()).unwrap();

    let res = client.broadcast(&signed_tx);
    dbg!(&res);
}
