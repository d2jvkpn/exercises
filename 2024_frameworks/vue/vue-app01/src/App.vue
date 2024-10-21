<script setup>
import { ref, computed } from 'vue';

import MyCounter from "./components/Ch03_MyCounter.vue";

// const init = 5;

const count = ref(0);
const doubleCount = computed(()=>count.value*2);
const increment = () => count.value++;

</script>

<template>
  <!--MyCounter :init="init+2" :end="20" :limits="{init:10, end:20}"/-->
  <!--MyCounter v-bind:init="init+2" end="20" v-bind:limits="{init:10, end:20}"/-->

  <br>
  <MyCounter :auto="false" :count="count" :doubleCount="doubleCount"/>

  <br><br>
  <button @click="increment()"> Button in App.vue count+1</button>
</template>

<style scoped>
</style>