<script setup>
  import { ref, computed, defineProps } from "vue"

  defineProps(["index"]);

  const timer = ref(null);
  const count = ref(0);
  const doubleCount = computed(() => count.value * 2);

  const start= () => timer.value = setInterval(() => count.value+=1, 1000);

  const stop = () => {
    if (timer.value) { clearInterval(timer.value); }
    timer.value = null;
  }
</script>

<template>
  <h3> {{index}} - MyCounter Component </h3>
  Reactive variable count: <b>{{count}}</b>

  <br>
  Computed variable doubleCount: <b>{{doubleCount}}</b>

  <br>
  <button v-if="!timer" @click="start()">Start</button>
  <button v-else @click="stop()">Stop</button>
</template>