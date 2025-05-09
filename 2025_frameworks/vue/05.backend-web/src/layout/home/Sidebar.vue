<script setup>
import { computed } from 'vue'
import { useRoute } from 'vue-router'

import { allRoutes } from '@/router/index'

const route = useRoute()

const props = defineProps({
  roles: Set,
})

//console.log(`--> role: ${role}`)
//console.log(`~~~ ${allRoutes.length}`);

const visibleRouteNames = computed(() => {
  const result = []

  const collect = (items) => {
    for (const item of items) {
      if (!item.name || !item.meta) { // only named routers
        continue
      }

      // console.log(`~~~ ${item.name}`)
      if (item.meta.roles?.includes("any") || item.meta.roles?.some(e => props.roles.has(e))) {
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
  <el-menu :default-active="$route.path" router>
    <el-menu-item index="/home/dashboard" v-if="visibleRouteNames.includes('Dashboard')">
      Dashboard
    </el-menu-item>

    <el-menu-item index="/home/accounts" v-if="visibleRouteNames.includes('Accounts')">
      Accounts
    </el-menu-item>

    <el-sub-menu index="/home/settings" v-if="visibleRouteNames.includes('Settings')">
      <template #title>Settings</template>
      <el-menu-item index="/home/settings/profile" v-if="visibleRouteNames.includes('Profile')">
        Profile
      </el-menu-item>

      <el-menu-item index="/home/settings/security" v-if="visibleRouteNames.includes('Security')">
        Security
      </el-menu-item>
    </el-sub-menu>
  </el-menu>
</aside>
</template>


<style scoped>
.sidebar {
  width: 15rem;
  color: #001219;
  padding-top: 10px;
  overflow-y: auto;
}
</style>
