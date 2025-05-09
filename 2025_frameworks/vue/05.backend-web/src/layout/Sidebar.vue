<script setup>
import { computed } from 'vue'
import { useRoute } from 'vue-router'

import { allRoutes } from '@/router/routes'

const route = useRoute()
const role = localStorage.getItem('role');
console.log(`--> role: ${role}`)

const visibleRouteNames = computed(() => {
  const result = []

  const collect = (items) => {
    for (const item of items) {
      if (!item.name || !item.meta) { // only named routers
        continue
      }

      // console.log(`~~~ ${item.name}`)
      if (item.meta.roles?.includes(role) || item.meta.roles?.includes("any")) {
        result.push(item.name)
      }

      if (item.children) {
        collect(item.children)
      }
    }
  }

  collect(allRoutes)
  // console.log(`--> ${JSON.stringify(result)}`)
  return result
})
</script>

<template>
<aside class="sidebar">
  <el-menu :default-active="$route.path" router> <!-- :collapse="true" -->
    <el-menu-item index="/dashboard" v-if="visibleRouteNames.includes('Dashboard')">
      Dashboard
    </el-menu-item>

    <el-menu-item index="/accounts" v-if="visibleRouteNames.includes('Accounts')">
      Accounts
    </el-menu-item>

    <el-sub-menu index="/settings" v-if="visibleRouteNames.includes('Settings')">
      <template #title>Settings</template>
      <el-menu-item index="/settings/profile" v-if="visibleRouteNames.includes('Profile')">
        Profile
      </el-menu-item>

      <el-menu-item index="/settings/security" v-if="visibleRouteNames.includes('Security')">
        Security
      </el-menu-item>
    </el-sub-menu>
  </el-menu>
</aside>
</template>


<style scoped>
.sidebar {
  width: 200px;
  color: #fff;
  padding-top: 10px;
  overflow-y: auto;
}
</style>
