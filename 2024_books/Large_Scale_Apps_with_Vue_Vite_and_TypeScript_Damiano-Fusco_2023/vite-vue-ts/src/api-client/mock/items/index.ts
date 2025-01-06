import {
  ItemsApiClientOptions,
  ItemsApiClientInterface,
  ItemsApiClientModel
} from '../../models/items';

const options: ItemsApiClientOptions = {
  endpoints: {
    fetchItems: '/static/mock-data/items/items.json'
  },
  mockDelay: 1000
}

const itemsApiClient: ItemsApiClientInterface = new ItemsApiClientModel(options)

export {
  itemsApiClient
}
