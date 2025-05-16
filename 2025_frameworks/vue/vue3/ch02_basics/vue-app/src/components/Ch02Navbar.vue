<script setup>
import { ref, computed } from "vue"

defineProps({
  activePage: { type: Number, default: 0 },
  navlinkClick: { type: Function, required: true },
  pages: {
    type: Object, default: [{
      link: { text: "Home", url: "index.html" },
      title: "Home Page",
      content: "This is the home contents.",
    }],
  },
})

const emit = defineEmits(['hello'])

const callHello = () => {
  emit('hello', "Navbar")
}

const useDarkNavbar = ref(false)

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
          class="nav-link emphasize"
          :class="{active: activePage == i}"
          aria-current="e"
          :href="e.link.url"
          :title="`this link goes to the ${e.link.text} page`"
          @click.prevent="navlinkClick(i); emit('hello', e.title)"
        > {{ e.link.text }} </a>
      </li>
    </ul>

    <form class="d-flex">
      <!--button class="btn btn-primary" @click="useDarkNavbar = !useDarkNavbar"-->
      <button class="btn btn-primary" @click.prevent="changeTheme"> Toggle </button>
    </form>
  </div>
</nav>
</template>


<style>
.emphasize {
  text-decoration: none;
}

.emphasize:active {
  color: skyblue !important;
  text-decoration: underline !important;
}
</style>