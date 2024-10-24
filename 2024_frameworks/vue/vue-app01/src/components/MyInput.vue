<script setup>
  import { ref, computed, onBeforeUpdate } from "vue";

  const count = ref(42);
  const doubleCount = computed(() => count.value * 2);

  onBeforeUpdate(() => console.log(`~~~ count: ${count.value}`));
</script>

<template>
  <h3> MyCounter Component </h3>
  Reactive variable count : <b>{{ count }}</b>

  <br>
  Computed variable doubleCount : <b>{{ doubleCount }}</b>

  <br><br>
  Input for count (using v-bind): <input type="text" v-bind:value="count" />

  <br><br>
  Input for count (using v-model): <input type="text" v-model="count" />
</template>
