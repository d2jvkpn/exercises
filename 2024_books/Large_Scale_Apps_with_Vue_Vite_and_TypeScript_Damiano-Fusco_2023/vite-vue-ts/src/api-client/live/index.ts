import { ApiClientInterface } from '../models'
// import module instances
import { itemsApiClient } from './items'

const apiLiveClient: ApiClientInterface = {
  items: itemsApiClient
}

export {
  apiLiveClient
}
