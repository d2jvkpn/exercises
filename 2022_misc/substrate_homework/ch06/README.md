### demo polkadot-api
---

#### install packages(Node.js v18.12.1)
```bash
npm install -g typescript ts-node yarn
yarn install
```

#### run
```bash
# ts-node main.ts
ts-node index.ts
```

output:
```text
>>> connection to substrate is OK
~~~ const value existentialDeposit is 500
--> Subscribed to alice account.
<-- Alice Account (sub): 1152874502981981099
<<< successfully exited

>>> Received 1 events:
--> system:ExtrinsicSuccess:: (phase={"applyExtrinsic":0})
~~~ event.data[0]
    type: {"weight":"SpWeightsWeightV2Weight","class":"FrameSupportDispatchDispatchClass","paysFee":"FrameSupportDispatchPays"}
    data: {"weight":{"refTime":260558000,"proofSize":0},"class":"Mandatory","paysFee":"Yes"}
```

#### note
- yarn add @polkadot/extension-dapp @polkadot/ui-keyring @polkadot/ui-settings
- console.log(api.genesisHash.toHex())
- api.consts, api.query, api.tx, api.rpc
- api.query.poeModule.getMaxClaimLength()

#### docs
- https://polkaddot.js.org/docs/api
- https://github.com/polkadot-js/api
- https://github.com/polkadot-js/docs/tree/master/docs/api
