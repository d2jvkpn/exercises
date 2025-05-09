<script setup>
import { ref, onMounted, watchEffect} from 'vue'
import { Fold, Expand, ArrowDown } from '@element-plus/icons-vue'

import Sidebar from "./Sidebar.vue"
import HeaderBar from "./HeaderBar.vue"

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
<div class="overview">
  <HeaderBar
    :accountName="accountName"
    :isSidebarHidden="isHidden"
    @toggleSidebar="isHidden = !isHidden"
  />

  <div class="overview-main">
    <Sidebar v-show="!isHidden" />

    <main class="overview-content">
      <router-view v-slot="{ Component }">
        <KeepAlive> <component :is="Component" /> </KeepAlive>
      </router-view>
    </main>
  </div>
</div>
</template>

<style scoped>
.overview {
  height: 100vh;
  display: flex;
  flex-direction: column;
}

.overview-main {
  display: flex;
  flex: 1;
  overflow: hidden;
}

.overview-content {
  position: relative;
  flex: 1;
  padding: 20px;
  background-color: #f5f7fa;
  overflow-y: auto;
}
</style>
