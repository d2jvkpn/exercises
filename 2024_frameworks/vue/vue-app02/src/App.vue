<script setup>
  import { ref, provide, onMounted } from "vue";

  import MyCountries from './components/MyCountries.vue';
  import MyCounter from './components/MyCounter.vue'

  const name = ref("");

  // onMounted(()=>document.querySelector("input[type=text]").focus());
  onMounted(() => document.querySelector("input[id=country]").focus());

  // 3. watch inject
  provide("name", name);
</script>

<template>
  <MyCounter />

  <hr>

  <b>Country</b>: <input type="text" id="country" v-model="name" />
  <br>

  <MyCountries :name="name" />
</template>
