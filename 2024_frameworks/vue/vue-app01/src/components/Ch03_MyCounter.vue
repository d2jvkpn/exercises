<script setup>
  import { defineProps } from "vue"

  const props = defineProps(["count", "doubleCount"]);
  console.log(`~~~ props:`, JSON.stringify(props));

  let doubleCount = 0; // override props.doubleCount
</script>

<template>
  <h3>MyCounter Component</h3>
  Reactive variable count : <b>{{ count }}</b>
  <br />
  Computed variable doubleCount : <b>{{ doubleCount }}</b>
  <br/>
  Input : <input type="text" v-bind:value="count" />
</template>