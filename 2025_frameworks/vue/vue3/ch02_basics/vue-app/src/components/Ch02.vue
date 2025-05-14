<script setup>
import { ref, computed } from "vue"

const useDarkNavbar = ref(false)
const activePage = ref(0)

const pages = ref([
  {
    link: { text: "Home", url: "index.html" },
    pageTitle: "Home Page",
    content: "This is the home contents.",
  },
  {
    link: { text: "About", url: "about.html" },
    pageTitle: "About Page",
    content: "This is the about contents.",
  },
  {
    link: { text: "Contact", url: "contact.html" },
    pageTitle: "Contact Page",
    content: "This is the contact contents.",
  },
])

const navbarClasses = computed(() => {
  return {
    'navbar-light bg-light': !useDarkNavbar.value,
    'navbar-dark bg-dark': useDarkNavbar.value,
  }
})

const theme = ref("light")

function changeTheme() {
  theme.value = theme.value == "light" ? "dark" : "light";
}
</script>


<template>
<!--nav
  class="navbar navbar-expand-lg"
  :class="{ 'navbar-light bg-light': !useDarkNavbar, 'navbar-dark bg-dark': useDarkNavbar }"
-->

<!--nav class="navbar navbar-expand-lg" :class="navbarClasses"-->

<nav class="navbar navbar-expand-lg" :class="[`navbar-${theme}`, `bg-${theme}`]">
  <div class="container-fluid">
    <!--a class="nav-brand" href="#"> My Vue </a-->

    <ul class="navbar-nav me-auto mb-2 mb-lg-0">
      <li v-for="(e, i) in pages" class="nav-item" :key="i">
        <a
          class="nav-link"
          :class="{active: activePage == i}"
          aria-current="e"
          :href="e.link.url"
          :title="`this link goes to the ${e.link.text} page`"
          @click.prevent="activePage = i"
        > {{ e.link.text }} </a>
      </li>
    </ul>

    <form class="d-flex">
      <!--button class="btn btn-primary" @click="useDarkNavbar = !useDarkNavbar"-->
      <button class="btn btn-primary" @click="changeTheme"> Toggle Navbar </button>
    </form>
  </div>
</nav>

<div class="containers content">
  <h4> {{ pages[activePage].pageTitle }} </h4>
  <p> {{ pages[activePage].content }} </p>
</div>
</template>

<style scoped>
.content {
  margin: 1rem 1rem;
}
</style>
