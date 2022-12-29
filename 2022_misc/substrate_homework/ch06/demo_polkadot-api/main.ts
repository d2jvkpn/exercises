// console.log("Hello, world!");
import { ApiPromise, WsProvider, Keyring } from "@polkadot/api";
import { KeyringPair } from "@polkadot/keyring/types";
import '@polkadot/api-augment';
import { metadata } from "@polkadot/types/interfaces/essentials";

const WEN_SOCKET = "ws://localhost:9944";
const sleep = (ms: number) => new Promise(resolve => setTimeout(resolve, ms));

// connect to substrate chain
const connectSubstrate = async () => {
  const wsProvider = new WsProvider(WEN_SOCKET);
  const api = await ApiPromise.create({provider: wsProvider, types: {}});
  await api.isReady;
  console.log(">>> connection to substrate is OK");
  return api
};

// get const value
const getConst = async (api: ApiPromise) => {
  const existentialDeposit = await api.consts.balances.existentialDeposit.toHuman();
  return existentialDeposit;
};

const getFreeBalance = async (api: ApiPromise, address: string) => {
  let { data: { free: previousFree }, nonce: previousNonce } =
    await api.query.system.account(address);

  console.log(`... getFreeBalance previousNonce: ${previousNonce}`);
  return previousFree.toHuman();
};

const printAliceBobBalance = async (api: ApiPromise) => {
  const keyring = new Keyring({type: "sr25519"});

  const alice = keyring.addFromUri("//Alice");
  const bob = keyring.addFromUri("//Bob");

  console.log("~~~ alice balance is:", await getFreeBalance(api, alice.address));
  console.log("~~~ bob balance is:", await getFreeBalance(api, bob.address));
};

const transferFromAliceToBob = async (api: ApiPromise, amount: number) => {
  const keyring = new Keyring({type: "sr25519"});

  const alice = keyring.addFromUri("//Alice");
  const bob = keyring.addFromUri("//Bob");

  await api.tx.balances.transfer(bob.address, amount)
    .signAndSend(alice, res => {
      console.log(`Tx status: ${res.status}`);
    });
};

const subscribeAliceBalance = async (api: ApiPromise) => {
  const keyring = new Keyring({type: "sr25519"});

  const alice = keyring.addFromUri("//Alice");
  await api.query.system.account(alice.address, account => {
    console.log("--> Subscribed to alice account.");
    const aliceFreeSub = account.data.free;
    console.log(`<-- Alice Account (sub): ${aliceFreeSub}`);
  });
};

const getMetadata = async (api: ApiPromise) => {
  const data = await api.rpc.state.getMetadata();
  console.log(`... print metadata: ${data}`);
  return data;
};

const main = async() => {
  const api = await connectSubstrate();
  console.log("~~~ const value existentialDeposit is", await getConst(api));
  // ~~~ const value existentialDeposit is 500

  // await printAliceBobBalance(api);
  // ~~~ alice balance is: 1,152,921,504,606,846,976
  // ~~~ bob balance is: 1,152,921,504,606,846,976

  // ~~~ alice balance is: 1,152,879,504,345,906,829
  // ~~~ bob balance is: 1,152,963,504,606,846,976

  // await transferFromAliceToBob(api, 10**12);
  // await sleep(6000);
  // await printAliceBobBalance(api);

  await subscribeAliceBalance(api);
  await sleep(6000);

  // await getMetadata(api);
  // console.log("<<< game over");
};

main().then(
  () => {
    console.log("<<< successfully exited");
    process.exit(0);
  }
).catch( err => {
  console.log("!!! error occur:", err);
  process.exit(1);
})
