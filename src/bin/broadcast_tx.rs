use bdk::bitcoin::consensus::encode::deserialize;
use bdk::bitcoin::Transaction;
use bdk_esplora::esplora_client;

fn main() {
    let client =
        esplora_client::Builder::new("http://esplora-api.bitlight-local-env.orb.local:3000")
            .build_blocking();

    let signed_tx_hex = "0200000000010150418fa568e8b07de632ebe69638cebbe0b537a715c5a0a5cd330f3fd4c71e310100000000000000000170dff50500000000225120f8a758e89a4180e637468c9b8ea6c30b2dc8beacb8cea2579f34541eb4d6d9e6014062634d95294f98609eb86b6a7b68e68beb559de41c2298b36c0416516a002d8bcf8f1c6efb19e2910d968552a0ba48463354109ee7eff4624fe3e8c7846bf83d00000000";
    let signed_tx: Transaction = deserialize(&hex::decode(signed_tx_hex).unwrap()).unwrap();

    let res = client.broadcast(&signed_tx);
    dbg!(&res);
}
