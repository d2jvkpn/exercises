<script setup>
  import * as vue from "vue";
  // import { ref } from "vue";

  import useMaximum from "../composables/useMaximum.js";
  import useFormatDate from "../composables/useFormatDate.js";
  import useUpperCase from "../composables/useUpperCase.js";

  // 1.
  console.log("==> setup: The component is created in memory.");

  const count = vue.ref(0);

  function increment() {
    count.value+=1;
  }

  const doubleCount = vue.computed(function() {
    return count.value * 2;
  });

  vue.onBeforeMount(() => console.log("==> onBeforeMount: The component is about to be mounted in the DOM."));
  vue.onMounted(() => console.log("==> onMounted: The component has been mounted in the DOM."));

  //vue.onBeforeUpdate(() => { console.log("==> onBeforeUpdate: The component is about to be updated."); });
  //vue.onUpdated(() => { console.log("==> onUpdated: The component has been updated."); });

  vue.onBeforeUnmount(() => { console.log("==> onBeforeUnmount: The component is about to be unmounted."); });
  vue.onUnmounted(() => { console.log("==> onUnmounted: The component has been unmounted."); });

  // 2.
  let timer;

  const start = () => {
    timer = setInterval(function() { increment(); }, 1000);
  };

  const stop = () => clearInterval(timer);

  vue.onMounted(() => start());
  vue.onUnmounted(() => stop());

  // 3.
  const maximum = 10;
  const c2 = useMaximum(maximum);

  function add() {
    c2.value += 1;
  }

  function minus() {
    c2.value -= 1;
  }

  // 4.
  const date= new Date(); // Current date
  const dateMMDDYYYY = useFormatDate(date, "MM-DD-YYYY");
  const dateDDMMYYYY = useFormatDate(date, "DD-MM-YYYY");
  const dateMMDDYYYY_slash = useFormatDate(date, "MM/DD/YYYY");
  const dateDDMMYYYY_slash = useFormatDate(date, "DD/MM/YYYY");

  // 5.
  const name = useUpperCase("eric");
</script>

<template>
  <h1> MyCounter Component </h1>

  Reactive variable count: <b>{{ count }}</b>
  <br>
  Computed variable doubleCount : <b>{{ doubleCount }}</b>
  <br>
  <button @click="count+=1">count+=1</button>
  <br>
  <button @click="increment">increase count</button>

  <br><br>
  Reactive variable c2: <b>{{ c2 }}</b>

  Maximum: <b>{{ maximum }}</b>

  <br><br>
  <button @click="add">add 1</button>
  &nbsp;
  <button @click="minus">minus 1</button>

  <hr>
  <h3> MyCounter Component </h3>
  Current date : {{date}}

  <br><br>
  Date (MM-DD-YYYY) : <b>{{ dateMMDDYYYY }}</b>
  <br><br>
  Date (DD-MM-YYYY) : <b>{{ dateDDMMYYYY }}</b>
  <br><br>
  Date (MM/DD/YYYY) : <b>{{ dateMMDDYYYY_slash }}</b>
  <br><br>
  Date (DD/M/YYYY) : <b>{{ dateDDMMYYYY_slash }}</b>
  <br><br>

  <hr>
  <h3> MyCounter Component </H3>
  <b>Uppercase Input:</b>
  <br><br>
  Variable name: <input type="text" v-model="name" />
  <br><br>
  Variable name: {{ name }}

</template>

<style scoped>
  h1 {
    font-family:papyrus;
    font-size:20px;
  }
</style>