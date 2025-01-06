<script setup lang="ts">
import ItemsView from '@/views/Items.view.vue';

const metaEnv = import.meta.env;
</script>

<template>
<div class="home">
  [{{metaEnv}}]

  <ItemsView />
</div>
</template>

<style>
.home {
  padding: 20px;
  font-family: Verdana, Geneva, Tahoma, sans-serif;
  font-size: 12px;
}
</style>
