<script setup>
import { ref, onBeforeMount } from "vue";

//import HelloWorld from './components/HelloWorld.vue'
import Ch02 from './components/Ch02.vue'

const config = ref({});

console.log(`==> import.meta.env: ${JSON.stringify(import.meta.env)}`)

onBeforeMount(() => {
  fetch(`/app.json`)
    .then(response => response.json())
    .then((data) => {
      config.value = data;
      console.log(`==> Got app.json: ${JSON.stringify(data)}`)
    })
    .catch(error => console.error(`!!! Error loading app.json: ${error}`));
});
</script>

<template>
  <!--div>
    <a href="https://vite.dev" target="_blank">
      <img src="/vite.svg" class="logo" alt="Vite logo" />
    </a>
    <a href="https://vuejs.org/" target="_blank">
      <img src="./assets/vue.svg" class="logo vue" alt="Vue logo" />
    </a>
  </div>
  <HelloWorld msg="Vite + Vue" /-->

<Ch02 />
</template>

<style scoped>
.logo {
  height: 6em;
  padding: 1.5em;
  will-change: filter;
  transition: filter 300ms;
}
.logo:hover {
  filter: drop-shadow(0 0 2em #646cffaa);
}
.logo.vue:hover {
  filter: drop-shadow(0 0 2em #42b883aa);
}
</style>
