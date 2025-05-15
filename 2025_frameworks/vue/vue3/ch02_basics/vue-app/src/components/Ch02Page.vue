<script setup>
/*
defineProps(["page"])
*/

const props = defineProps({
  page: {
    type: Object,
    default: { title: "Default Page Title", content: "Default page content" },
  },
})

console.log(`??? ${props.page.title}`)
</script>

<template>
<div class="containers content">
  <!--
  <h4> {{ pages[activePage].title }} </h4>
  <p> {{ pages[activePage].content }} </p>
  -->

  <h4> {{ props.page.title }} </h4>
  <p> {{ props.page.content }} </p>
</div>
</template>

<style scoped>
.content {
  margin: 1rem 1rem;
}
</style>
