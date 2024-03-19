//! A simple program to be proven inside the zkVM.

#![no_main]
sp1_zkvm::entrypoint!(main);

use poly_milestone::proof::Proof;

fn test_milestone() {
    let signer_addrs = Proof::check_milestone_proof(
        "d779a24664fd354017a7469607dee3faa87ef4978112d1c8e958b2a5ec7d52f4".to_string(),
        "0000000000000000000000004ad84f7014b7b44f723f284a85b166233797143900000000000000000000000000000000000000000000000000000000003b528500000000000000000000000000000000000000000000000000000000003b52926f73bdeda24c8d6b978628e10c425f5a8bbf181a547dafdf5eb156135626728e00000000000000000000000000000000000000000000000000000000000138820000000000000000000000000000000000000000000000000000000000000000".to_string(),
        vec![
            "f9025ba0ca10bf945143605003055ada553e7ea0c21209d29504ec2178ad87b17eb9ae45a01dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347940000000000000000000000000000000000000000a0d779a24664fd354017a7469607dee3faa87ef4978112d1c8e958b2a5ec7d52f4a056e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421a056e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421b901000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000008833b52908401c9c380808465da4086b861d88301010083626f7289676f312e32302e3132856c696e757800000000000000df4cd8299e10945f55652178c6d97eed031f38cf7e64bf7e7aa92cbef0be18c7720c89f91555c7caa8545e6f698124c6d8acacc734befcb5ef51571b8bb119ea01a000000000000000000000000000000000000000000000000000000000000000008800000000000000000f".to_string(),
            "f9025ba0580d9d7fe773a0bfad41cd99298caec77fc1dc2ef8a9146e880efc0ae12d05c3a01dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347940000000000000000000000000000000000000000a0d779a24664fd354017a7469607dee3faa87ef4978112d1c8e958b2a5ec7d52f4a056e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421a056e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421b901000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000008833b52918401c9c380808465da4088b861d88301010083626f7289676f312e32302e3132856c696e757800000000000000856ccbe51c6e4890c18b1c424c2b48e995609e4c58e3f17bced5172ab9f01bdc2022df7e8e19090163f1973e0b3319efa8136ed15ad6b0e4c16f839ed8a8d52501a000000000000000000000000000000000000000000000000000000000000000008800000000000000000f".to_string(),
            "f9025ba0a530320dccbb373627ab24c62ea6109ec90316770dd9f11c785515bd52036c2aa01dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347940000000000000000000000000000000000000000a0d779a24664fd354017a7469607dee3faa87ef4978112d1c8e958b2a5ec7d52f4a056e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421a056e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421b901000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000008833b52928401c9c380808465da408ab861d88301010083626f7289676f312e32302e3132856c696e7578000000000000006fd62d4bef9b20517551ed97587d0534253a0f326144c757abdbd712ac6d0313447356c442b295ff11d40b8ec51236c6e51342f6f9d5b6945207238efc17142501a000000000000000000000000000000000000000000000000000000000000000008800000000000000000f".to_string()
        ],
        vec![
            "c1bf75b75f5324ff2fca1edfed82ad81f75047f0b371c6196b13b47b59311539005ecefdd499f9ff4ad1cedbddda3df7776f387a62337a77f0109756fa7f85e300".to_string(),
            "ddcf586ab6caeb8a3fad37d523b0913cc799a955adb3c8e33665b9b70dca5b9c1981ae6d33d9d70259d4758ab801cc74f24e1ec149d16dc96a8e68743ea26c1901".to_string(),
            "1364deca1afde0644bdca4fdfc07a0e0f6f9071bbfca0ec52d5655bdd72bfe5d04c9617016302244f6369c19cabe1709d08970461fbfe3c1509410ee87615e1c01".to_string(),
            "12cceac8adc43c0768780b2051bbabe9279d9c36e2a419647cf4515c2e55a43416b66e002a881a9c27cefcc7714a18b6821f451d3a7c04ce217e48297cdfa22700".to_string(),
            "4343d782234737e5cb36866a0a2238a08d7f9fec9a3659fe28e0620ff72afe503cd8d2352fc26f84fb0f09e76784c4e7fb44141325a046ae4d11180bc21e1f7c00".to_string(),
            "ec55f6e732525df45c2a9a4b04111384c67c30d91a54cd59de313749666b2038615a6907d5ea4761f3049d77961a1063a65760fdba3736c110229b4e2faba0fe01".to_string(),
            "32f2a7ee00f089c49c79ca8a9615278586b60385bb4bd8d0a23ee9a99fae315a3a304d0f21047c35ee4ba2d69d7fb4a23e68f24c70dafb73828e6a1d7ee4d66901".to_string(),
        ],
    );
}

fn test_invariant() {
    poly_invariant::proof::Proof::do_balance_test(1000);
}

pub fn main() {
    // let headers: Vec<String> = sp1_zkvm::io::read();
    test_invariant()
    // sp1_zkvm::io::write(&signer_addrs.unwrap());
}