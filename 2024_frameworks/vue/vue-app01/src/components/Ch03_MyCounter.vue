<script setup>
import { defineProps, onUpdated, ref } from "vue"

  const props = defineProps(["count", "doubleCount"]);
  console.log(`~~~ props:`, JSON.stringify(props));

  let doubleCount = 0; // override props.doubleCount

  let isVisible = ref(true);
  const fn01 = () => isVisible.value = !isVisible.value;

  onUpdated(() => console.log("~~~ updated: count = ", props.count));
</script>

<template>
  <h3>MyCounter Component</h3>
  Reactive variable count : <b>{{ count }}</b>

  <br>
  Computed variable doubleCount : <b>{{ doubleCount }}</b>

  <br>
  Input : <input type="text" v-bind:value="count" />

  <br><br>
  &nbsp;
  <button v-if="isVisible" @click="fn01()">OK</button>
  <!--p v-else-if="xx"> Other </p -->
  <button v-else  @click="fn01()">No</button>

  <p v-show="isVisible">Show</p>

</template>