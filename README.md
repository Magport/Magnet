
## What is the MAGNET?


**MAGNET serves as Polkadot's Smart Contract Docking Station, operating on the PAYG Model. It is in the process of developing a scalable smart contract platform, utilizing DOT as the gas fee within this model.**

🔎 For more about Magnet Network, head to our [website](https://magnet.magport.io/)<br>
📢 Follow our latest updates on [Twitter](https://twitter.com/Magnet20239)<br>
🤝 Engage with fellow developers on our [Telegram server](https://t.me/+aep298N0KXUwZTM1)<br>

### Business Model Introduction

The main roles involved in the business logic are as follows:

**Tanssi:** As Magnet will become a ContainerChain in the
Tanssi Network, Collators will be elected and supported by the Tanssi
Network, and Magnet will no longer set up a Collators election
mechanism.

**Magnet:** As a smart contract platform, it provides EVM/WASM smart
contract services and charges GAS fees as platform fees.

**Polkadot:** Provides high-quality BlockSpace for Magnet and offers
blockchain security services.

![Diagram of Magnet Operation](https://github.com/Magport/Magnet/blob/main/Img/DiagramofMagnetOperation.png)

When providing smart contract platform services to users, Magnet calculates the total GAS fees from the current transaction pool and obtains the expected Blockspace/coretime price from RelayChain. By strategically leveraging the price difference, Magnet achieves continuous profits, thereby serving users more efficiently. Once the GAS fees exceed the expected price, Magnet will delegate Collators to package the transactions and submit a Blockspace Order to the Relaychain, strategically collecting gas fees to pay for coretime. Upon successful ordering, the produced blocks will be anchored to the Relaychain for block confirmation.

## Magnet pallet introduction

### Blockspace Ordering Pallet:

This pallet is crucial for managing and monitoring various aspects of blockspace and transaction processing. 
1. **Monitor Pool Gas:**  Constantly observes the gas in the pool to ensure smooth transaction processing and to avoid any bottlenecks. 
2. **Monitor Polkadot CoreTime Price:**  Keeps track of the real-time price of Polkadot CoreTime to facilitate accurate and fair bidding. 
3. **Bidder Triggered, Transaction Added to Block, Block Production, Block Submission, Block Verification Locked:**  This is a multi-step process where a bidder triggers the addition of a transaction to a block. After this, the block goes through production, submission, and a verification lock to ensure integrity and security. 
4. **Gas Fee Revenue Credited to System Account:**  The revenue generated from gas fees is credited directly to the system account, maintaining a transparent financial flow. 
5. **Invoke Profit Operator:**  Calls the Profit Operator pallet to confirm CoreTime cost and calculate block profit.
### Profit Operator Pallet:

This pallet is responsible for managing profits and costs associated with CoreTime. 
1. **Confirm CoreTime cost:**  Validates the cost associated with CoreTime to avoid any discrepancies. 
2. **Calculate Block_Profit:**  Determines the profit derived from each block considering various parameters and metrics. 
3. **Incremental Era_Cost:**  Calculates the incremental cost associated with each era to manage financial flow effectively. 
4. **Incremental Era_Profit:**  Computes the incremental profit for each era, providing a clear information of financial gains. 
5. **Profit Distribution:**  Distributes the profit based on the incremental Era_Profit value, withdrawing it from the system account. Negative numbers are not triggered to avoid financial discrepancies. 
6. **Deposit Credit account based on the Incremental Era_Cost Value:**  Credits the account based on the calculated incremental Era_Cost value, ensuring financial accuracy. 
7. **Triggered every era, or Triggered by other module:**  This pallet is invoked at every era or can be triggered by another module when needed.
### Blockspace/Coretime Assurance Pallet:

This pallet ensures the stability and reliability of block intervals and credit account balances. 
1. **Monitor Block Interval Time:**  Observes the time intervals between blocks to maintain a consistent and efficient block production rate. 
2. **Monitor Credit Account Balance:**  Keeps a close watch on the credit account balance to avoid any financial irregularities or issues. 
3. **Trigger Order or Profit Distribution Upon Reaching Threshold:**  Initiates order or profit distribution processes once a certain threshold is reached, ensuring smooth and timely operations.

Each of these pallets plays a pivotal role in maintaining the efficiency, security, and financial integrity of the system, working in tandem to ensure the seamless functioning of the blockchain network.


## Installing Substrate Dependencies

Substrate is a modular framework that enables you to create purpose-built blockchains by composing custom or pre-built components. Below is a markdown document that you can use as a template for creating a guide on installing Substrate dependencies.
### 1. Install Rust

Substrate is developed using Rust; hence, Rust is a prerequisite for Substrate. Install Rust using rustup by running the following command in your terminal:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```



After completing the installation, restart your terminal and run:

```sh
source $HOME/.cargo/env
```



Or, add the following line to your shell profile file (e.g., `~/.bashrc` or `~/.zshrc`):

```sh
export PATH=$HOME/.cargo/bin:$PATH
```


#### Update Rust

Keep your Rust installation up to date by running:

```sh
rustup update
```


### 2. Install Additional Libraries

Substrate has several library dependencies. Install them using the appropriate commands for your operating system:
#### For Ubuntu

```sh
sudo apt update
sudo apt install -y cmake pkg-config libssl-dev git build-essential clang libclang-dev
```


#### For macOS

```sh
brew install cmake pkg-config openssl git llvm
```


### 3. Install Substrate

With Rust and the necessary libraries installed, proceed to install Substrate:

```sh
curl https://getsubstrate.io -sSf | bash -s -- --fast
```


### 4. Verify Installation

Check your Substrate installation by running:

```sh
substrate --version
```



This command should output the installed Substrate version.
### 5. Configure Rust Toolchain

Configure the Rust toolchain for Substrate by running:

```sh
rustup default nightly
rustup target add wasm32-unknown-unknown --toolchain nightly
```


### Completion

At this point, you should have a working Substrate development environment. Regularly check for updates and keep your installations current by running `rustup update` and `cargo update`.

Remember to replace the placeholder text with the actual content, and feel free to modify the structure and formatting to suit your preferences and requirements.


## Start the Magnet Node

To start Magnet, you will need a proper Substrate development environment.

If you need a refresher setting up your Substrate environment, see [Substrate's Getting Started Guide](https://substrate.dev/docs/en/knowledgebase/getting-started/).

### I. Launching the Local Relay Chain

Using the   [`custom-spec-raw.json`](https://docs.substrate.io/assets/tutorials/relay-chain-specs/raw-local-chainspec.json/) file provided by substrate.
#### 1.Build the relay chain node
Clone the most recent release branch of the Polkadot repository to prepare a stable working environment.

```sh
git clone --branch release-v1.1.0 https://github.com/paritytech/polkadot-sdk.git
```
Change to the root of the polkadot directory by running the following command:

```sh
cd polkadot
```

Build the relay chain node by running the following command:
```sh
cargo build --release
```
Compiling the node can take 15 to 60 minuets to complete.

Verify the node built correctly by running the following command:

```sh
./target/release/polkadot --help
```
If command-line help is displayed, the node is ready to configure.


#### 2. Create the First Node

```sh
nohup ./target/release/polkadot --alice --validator --chain custom-spec-raw.json --port 30333 --rpc-port 9944  --rpc-cors all --unsafe-rpc-external >alice.log 2>&1 &
```


#### 3. Create the Second Node

```sh
nohup ./target/release/polkadot --bob --validator --chain custom-spec-raw.json --port 30334 --rpc-port 9945 --rpc-cors all --unsafe-rpc-external >bob.log 2>&1 &
```


### II. Connecting to the Local Parachain 
#### 1.Build the Magnet node

```sh
git clone https://github.com/Magport/Magnet.git

cd Magnet

cargo build --release
```
Compiling the node can take 30 minuets to complete.
#### 2. Create a new `paraid` in the browser 
- a. Click on `network`, select `parachains`. 
- b. Click on `parathreads`, click on `paraid`.
- c. Choose an account and submit. 
- d. The registered `paraid` for this session is 2000.
#### 3. Modify the Default Chain Specification
- a. Generate the default chain specification:

```sh
./target/release/magnet build-spec --disable-default-bootnode  --chain=dev  >custom-parachain-spec.json
```


Modify the `custom-parachain-spec.json` file, change `para_id` to 2000 and `parachainid` to 2000.
- b. Convert the spec file to a raw file:

```sh
./target/release/magnet build-spec --chain custom-parachain-spec.json  --disable-default-bootnode --raw > custom-parachain-spec-raw.json
```


#### 4. Prepare the Parachain Collator
- a. Export the wasm file:

```sh
./target/release/magnet export-genesis-wasm --chain ./custom-parachain-spec-raw.json para-2000-wasm
```


- b. Generate the genesis state of the parachain:

```sh
./target/release/magnet  export-genesis-state --chain ./custom-parachain-spec-raw.json para-2000-genesis-state
```


- c. Start the collator node:

```sh
Nohow ./target/release/magnet --alice --collator --force-authoring --chain custom-parachain-spec-raw.json --base-path /data/zachary/alice/ --port 40333 --rpc-port 8844 --rpc-cors all --unsafe-rpc-external -- --execution wasm --chain ../polkadot-sdk/custom-spec-raw.json  --port 30343 --rpc-port 9977 > log.log 2>&1 &
```


#### 5. Register on the Local Relay Chain 
- a. Open the browser, click on `Developer`, select `Sudo`. 
- b. On the left, select `paraSudoWrapper`, on the right, select `sudoScheduleParaInitialize(id,genesis)`. 
- c. For `id`, enter 2000.
For `genesisHead`, choose file upload and upload the genesis file `para-2000-genesis-state` generated in the steps above.
For `ValidationCode`, choose file upload and upload the file `para-2000-wasm` generated above.
For `paraKind`, select yes.

Click `submit`, followed by `sign and submit`. Then, check the explorer to verify that the parachain is syncing with the relay chain.
#### 6. Test Parachain Block Production

Using polkadot.js, connect to port 8844, initiate a transfer, and verify that blocks are generated as expected.


## License

Licensed under the [Apache License, Version
2.0](./LICENSE).

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this repository by you, as defined in the Apache-2.0 license, shall be
licensed as above, without any additional terms or conditions.