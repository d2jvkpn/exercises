<script setup>
import { ref, computed } from "vue"

import Navbar from "./Ch02Navbar.vue"
import Page from "./Ch02Page.vue"

const activePage = ref(0)

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
</script>

<template>
<!--nav
  class="navbar navbar-expand-lg"
  :class="{ 'navbar-light bg-light': !useDarkNavbar, 'navbar-dark bg-dark': useDarkNavbar }"
-->

<!--nav class="navbar navbar-expand-lg" :class="navbarClasses"-->


<Navbar
  :pages="pages" :activePage="activePage"
  :navlinkClick="(i) => activePage = i"
  @hello="(name) => console.log(`==> Hello, ${name}!`)"
/>
<Page :page="pages[activePage]" />

</template>