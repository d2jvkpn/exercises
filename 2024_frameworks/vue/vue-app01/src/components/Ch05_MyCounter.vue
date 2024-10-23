<script setup>
  import { ref, defineProps } from "vue"

  const props = defineProps(["limit", "index"]);
  const init = props.limit.init || 0;
  const end = props.limit.end || 42;

  const timer = ref(null);
  const count = ref(init);
  // const doubleCount = computed(() => count.value * 2);

  const increment = () => {
    if (count.value < end) {
      count.value+=1;
    }
  };

  const start= () => {
    timer.value = setInterval(() => increment(), 1000);
  }

  const stop = () => {
    clearInterval(timer.value);
    timer.value = null;
  }
</script>

<template>
  <h3> {{index}} - MyCounter Component </h3>
  init = {{init}}, end = {{end ? end : "infinity"}}
  <br />
  Reactive variable count : <b>{{ count }}</b>
  <br /><br />
  <button v-if="!timer" @click="start()">Start</button>
  <button v-else @click="stop()">Stop</button>
</template>