<script setup>
  import MyCounter from "./components/Ch05_MyCounter.vue";

  const limits = [
    {init: 0, end: 10},
    {init: 5},
    {end: 10},
  ];
</script>

<template>
  <MyCounter v-for="(limit, i) in limits" :key="i" :index="i+1" :limit="limit" />
</template>