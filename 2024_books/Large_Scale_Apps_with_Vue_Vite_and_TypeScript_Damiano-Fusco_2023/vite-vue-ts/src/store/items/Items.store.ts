import { reactive } from 'vue';

import { ItemsStateInterface } from './models';
import { ItemInterface } from '@/models';

const itemsState = reactive<ItemsStateInterface>({
  loading: false,
  items: [],
})

import { apiClient } from '@/api-client';

const actions = {
  // action that we invoke to load the items from an api:
  loadItems: async () => {

    // set loading to true and clear current data:
    itemsState.loading = true
    itemsState.items = []

    const data = await apiClient.items.fetchItems()
    itemsState.items = data
    itemsState.loading = false
  },

  toggleItemSelected: async (id: number) => {
    const item = (itemsState.items || []).find((o) => o.id === id)
    if (item) {
      item.selected = !item.selected
    }
  }
}

const getters = {
  get loading() {
    return itemsState.loading
  },
  get items() {
    return itemsState.items
  }
}

export interface ItemsStoreInterface {
  getters: typeof getters
  actions: typeof actions
}

export function useItemsStore(): ItemsStoreInterface {
  return { getters, actions}
}
