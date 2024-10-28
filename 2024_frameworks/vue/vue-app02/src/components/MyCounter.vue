<script setup>
  import { ref } from "vue";

  const count = ref("123abcDEF");
</script>

<template>
  <h3>MyCounter Component</h3>
  Reactive variable count: <input type="text" v-focus v-integers-only.hexa.upper v-model="count" />
  <br/><br/>

  Entered value: <b>{{count}}</b>
  <br><br>
</template>
