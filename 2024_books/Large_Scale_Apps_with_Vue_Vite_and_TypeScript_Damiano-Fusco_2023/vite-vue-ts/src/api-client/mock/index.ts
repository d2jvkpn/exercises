import { ApiClientInterface } from '../models/ApiClient.interface'
import { itemsApiClient } from './items'

const apiMockClient: ApiClientInterface = {
  items: itemsApiClient
}

export {
  apiMockClient
}
