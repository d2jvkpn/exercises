export interface ItemsApiClientEndpoints {
  fetchItems: string
}

export interface ItemsApiClientOptions {
  mockDelay?: number
  endpoints: ItemsApiClientEndpoints
}
