<script setup>
import { ref, computed, onMounted, onUnmounted, defineProps } from 'vue';

const props = defineProps(["init", "end", "auto", "limits"]);
// console.log("~~~", typeof(props.init));
console.log(`~~~ limits:`, JSON.stringify(props.limits));

//
const params = {
  init: parseInt(props.init) || 0,
  end: parseInt(props.end) || 0,
  auto: String(props.auto) != "false",
};

const count = ref(params.init);
const doubleCount = computed(() => count.value * 2);

//
let timer;

const start = () => {
  timer = setInterval(() => {
    increment();
  }, 1000);
};

const stop = () => clearInterval(timer);

const increment = () => {
  if (!params.end || count.value < params.end) {
    count.value++;
  } else {
    stop();
  }
};

if (params.auto) {
  onMounted(() => start());
  onUnmounted(() => stop());
}

</script>

<template>
  <h3>MyCounter Component</h3>
  init: {{params.init}} => end: {{params.end}}
  <br/><br>
  Reactive variable count: <b>{{count}}</b>
  <br />
  Computed variable doubleCount: <b>{{doubleCount}}</b>

  <br/><br/>
  Input1 : <input type="text" value="{{count}}" />
  <br>

  Input2 : <input type="text" :value="count" />

  <br>
  Input3 : <input type="text" v-bind:value="count+1" />

  <br>
  <button @click="increment()">MyCounter count+1</button>
</template>

<style scoped>
</style>