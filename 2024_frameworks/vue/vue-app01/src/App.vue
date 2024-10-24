<script setup>
  import MyCounter from "./components/Ch05_MyCounter.vue";
  import MyInput from "./components/MyInput.vue";
  import MyForm from "./components/MyForm.vue";
  import MyEvents from "./components/MyEvents.vue";

  const limits = [
    {init: 0, end: 10},
    {init: 5},
    {end: 10},
  ];
</script>

<template>
  <MyCounter v-for="(limit, i) in limits" :key="i" :index="i+1" :limit="limit" />

  <MyInput/>

  <MyForm/>

  <MyEvents/>
</template>
