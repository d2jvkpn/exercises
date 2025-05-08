<script setup>
import { computed } from 'vue'
import { useRoute } from 'vue-router'

import Overview from './components/Overview.vue'


const route = useRoute()

const layoutComponent = computed(() => {
  const layout = route.meta.layout

  if (layout === 'admin') {
    return Overview
  }

  return 'div';
})
</script>


<template>
  <component :is="layoutComponent">
    <router-view />
  </component>
</template>
