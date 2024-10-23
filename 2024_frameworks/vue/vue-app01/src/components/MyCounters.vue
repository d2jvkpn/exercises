<script setup>
  import MyCounter from "./Ch04_MyCounter.vue";
  import { defineProps } from 'vue';

  defineProps(["nb"]);
</script>

<template>
  <MyCounter v-for="i in nb" :key="i" :index="i" />
</template>