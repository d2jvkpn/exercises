<script setup lang="ts">
  // expose a property called items with a default value of a blank array
  defineProps<{ items: any[] }>();

  // explicetely using any[] as we'll replace this with an interface in the next chapters
  const emit = defineEmits<{(e: 'selectItem', id: number): any}>();

  /*
  const handleClick = (item: ItemInterface) => {
    item.selected = !item.selected;
    console.log(`--> Handle Item Click: ${item.id}, ${ item.selected}`);
  };
  */

  const handleClick = (item: ItemInterface) => {
    emit('selectItem', item.id); // <-- modify the body of the method by just emitting "itemClicked"
  };
</script>

<template>
  <div>
    <h3>Items:</h3>
    <ul>
      <li v-for="(item, id) in items" :key="item.id" @click="handleClick(item)">
        {{item.name}}: {{item.selected}}
      </li>
    </ul>
  </div>
</template>
