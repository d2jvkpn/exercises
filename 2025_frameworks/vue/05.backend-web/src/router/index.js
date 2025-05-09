import { createRouter, createWebHistory } from 'vue-router'
import { allRoutes } from './routes'

const router = createRouter({
  history: createWebHistory(import.meta.env.VITE_BASE_PATH),
  routes: allRoutes,
})

router.beforeEach((to, from, next) => {
  document.title = String(to.meta.title) || 'Page Not Found';

  const isLoggedIn = !!localStorage.getItem('token');

  if (to.meta.layout === 'overview' && !isLoggedIn) {
    next('/login')
  } else {
    next()
  }
})

export default router
