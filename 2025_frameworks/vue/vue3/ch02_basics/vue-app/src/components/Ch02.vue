<script setup>
import { ref, computed, onBeforeMount } from "vue"

import Navbar from "./Ch02Navbar.vue"
import Page from "./Ch02Page.vue"

const activePage = ref(0)

/*
const pages = ref([
  {
    link: { text: "Home", url: "index.html" },
    title: "Home Page",
    content: "This is the home contents.",
  },
  {
    link: { text: "About", url: "about.html" },
    title: "About Page",
    content: "This is the about contents.",
  },
  {
    link: { text: "Contact", url: "contact.html" },
    title: "Contact Page",
    content: "This is the contact contents.",
  },
])
*/


const pages = ref([]);

onBeforeMount(() => {
  fetch("pages.json")
    .then(response => response.json())
    .then(data => pages.value = data)
    .catch(error => console.error(`!!! Error loading pages: ${error}`));
});

</script>

<template>

<Navbar
  :pages="pages" :activePage="activePage"
  :navlinkClick="(i) => activePage = i"
  @hello="(v) => console.log(`==> Hello, ${v}!`)"
/>

<Page v-if="pages.length > 0" :page="pages[activePage]" />
<!--Page v-show="pages.length > 0" :page="pages[activePage]" /-->

</template>
