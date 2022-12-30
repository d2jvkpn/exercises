// https://github.com/polkadot-js/docs/blob/master/docs/api/examples/promise/system-events.md

// Import the API
import { ApiPromise, WsProvider, Keyring } from "@polkadot/api";
import { EventRecord } from '@polkadot/types/interfaces';

const WEN_SOCKET = "ws://localhost:9944";

async function main () {
  const wsProvider = new WsProvider(WEN_SOCKET);
  // Create our API with a default connection to the local node
  const api = await ApiPromise.create({provider: wsProvider});

  // Subscribe to system events via storage
  api.query.system.events((events: any[]) => {
    console.log(`>>> Received ${events.length} events:`);

    // Loop through the Vec<EventRecord>
    events.forEach((record: EventRecord) => {
      // Extract the phase, event and the event types
      const { event, phase } = record;
      const types = event.typeDef;

      // Show what we are busy with
      console.log(`--> ${event.section}:${event.method}:: (phase=${phase.toString()})`);
      // console.log(`\t\t${event.meta.documentation.toString()}`);

      // Loop through each of the parameters, displaying the type and data
      event.data.forEach((data, index) => {
        console.log(`~~~ event.data[${index}]\n    type: ${types[index].type}` +
          `\n    data: ${data.toString()}`);
      });
    });
  });
}

main().catch((error) => {
  console.error(`!!! ${error}`);
  process.exit(-1);
});
