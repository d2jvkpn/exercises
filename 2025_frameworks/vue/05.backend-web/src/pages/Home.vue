<script setup>
import { ref, onMounted, watchEffect} from 'vue'
import { Fold, Expand, ArrowDown } from '@element-plus/icons-vue'

import Sidebar from "../layout/home/Sidebar.vue"
import HeaderBar from "../layout/home/HeaderBar.vue"

//
const accountName = localStorage.getItem('accountName')

//
const isHidden = ref(false)

onMounted(() => {
  const mediaQuery = window.matchMedia('(max-width: 720px)')

  isHidden.value = mediaQuery.matches

  mediaQuery.addEventListener('change', (e) => {
    isHidden.value = e.matches
  })
})

const toggleCollapse = () => {
  isHidden.value = !isHidden.value
}
</script>


<template>
<div class="home">
  <HeaderBar
    :accountName="accountName"
    :isSidebarHidden="isHidden"
    @toggleSidebar="isHidden = !isHidden"
  />

  <div class="home-main">
    <Sidebar v-show="!isHidden" />

    <main class="home-content">
      <router-view v-slot="{ Component }">
        <KeepAlive> <component :is="Component" /> </KeepAlive>
      </router-view>
    </main>
  </div>
</div>
</template>


<style scoped>
.home {
  height: 100vh;
  display: flex;
  flex-direction: column;
}

.home-main {
  display: flex;
  flex: 1;
  overflow: hidden;
}

.home-content {
  position: relative;
  flex: 1;
  padding: 20px;
  background-color: #f5f7fa;
  overflow-y: auto;
}
</style>
