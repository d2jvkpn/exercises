<script setup>
  import { ref } from "vue";

  const count = ref("123abcDEF");
  const max = 200;
  const color = "cyan";
</script>

<template>
  <h3>MyCounter Component</h3>
  Reactive variable count:

  <input type="text"
    v-focus:color="color"
    v-integers-only.hexa.upper
    v-max-value.bold="max"
    v-model="count"
  />

  <br/><br/>
  Max value: <b>{{max}}</b>

  <br/><br/>

  Entered value: <b>{{count}}</b>
  <br><br>
</template>
