import { ApiClientInterface } from './models';
import { apiMockClient } from './mock';
import { apiLiveClient } from './live';

let env: string = 'mock';

if (import.meta.env && import.meta.env.VITE_API_CLIENT) {
  env = import.meta.env.VITE_API_CLIENT.trim()
}

let apiClient: ApiClientInterface

if (env === 'live') {
  apiClient = apiLiveClient
} else {
  // default is always apiMockClient
  apiClient = apiMockClient
}

export {
  apiClient
}
