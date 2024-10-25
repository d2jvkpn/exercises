<script setup>
  import MyCountries from './components/MyCountries.vue'
  import { ref, provide } from "vue";

  const name = ref("");

  // 3. watch inject
  provide("name", name);
</script>

<template>
  <b>Country</b>: <input type="text" v-model="name" />
  <br>

  <MyCountries :name="name" />
</template>
