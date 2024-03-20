<script setup lang="ts">
  import { reactive } from 'vue';

  import ItemsListComponent from './components/items/ItemsList.component.vue';
  import type { ItemInterface } from './models/items/Item.interface';

  const items: ItemInterface[] = reactive([
    { id: 1, name: 'Item 1', selected: false },
    { id: 2, name: 'Item 2', selected: false },
    { id: 3, name: 'Item 3', selected: false },
  ]);

  const onSelectItem = (id: number) => {
    // retrieve the item from our local data
    const item = items.find(o => o.id === id);

    // sanity check:
    if (!item) {
      console.warn(`!!! onSelectItem: could not find item with id: ${id}`);
      return;
    }

    // update the item property
    item.selected = !item.selected;
    console.log(`--> onSelectItem: id=${item.id}, selected=${item.selected}`);
  };
</script>

<template>
  <div class="home">
    <ItemsListComponent :items="items" @selectItem="onSelectItem"/>
  </div>
</template>

<!--script setup lang="ts">
  import HelloWorld from './components/HelloWorld.vue'
</script-->

<!--template>
  <div>
    <a href="https://vitejs.dev" target="_blank">
      <img src="/vite.svg" class="logo" alt="Vite logo" />
    </a>
    <a href="https://vuejs.org/" target="_blank">
      <img src="./assets/vue.svg" class="logo vue" alt="Vue logo" />
    </a>
  </div>
  <HelloWorld msg="Vite + Vue" />
</template>

<style scoped>
.logo {
  height: 6em;
  padding: 1.5em;
  will-change: filter;
  transition: filter 300ms;
}
.logo:hover {
  filter: drop-shadow(0 0 2em #646cffaa);
}
.logo.vue:hover {
  filter: drop-shadow(0 0 2em #42b883aa);
}
</style-->
