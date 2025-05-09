import { createRouter, createWebHistory } from 'vue-router'
import PageNotFound from '@/pages/PageNotFound.vue'

export const allRoutes = [
  {
    path: '/login',
    name: "Login",
    meta: { title: "Login", layout: 'none', roles: ["Any"] },
    component: () => import('@/pages/Login.vue'),
  },

  {
    path: '/dashboard',
    name: "Dashboard",
    meta: { title: "Dashboard", layout: 'overview', roles: ["editor", "admin"] },
    component: () => import('@/layout/Dashboard.vue'),
  },

  {
    path: '/accounts',
    name: "Accounts",
    meta: { title: "Accounts", layout: 'overview', roles: ["admin"] },
    component: () => import('@/layout/Accounts.vue'),
  },

  {
    path: '/settings',
    name: "Settings",
    meta: { title: "Settings", layout: 'overview', roles: ["Any"] },
    // component: () => import('@/layout/Settings.vue'),
    children: [
      {
        path: 'profile',
        name: "Profile",
        meta: { title: "Settings / Profile", layout: 'overview', roles: ["editor"] },
        component: () => import('@/layout/settings/Profile.vue'),
      },
      {
        path: 'security',
        name: "Security",
        meta: { title: "Settings / Security", layout: 'overview', roles: ["admin"] },
        component: () => import('@/layout/settings/Security.vue'),
      },
    ],
  },

  {
    path: '/page-not-found',
    name: "PageNotFound",
    meta: { title: "Page not found", layout: 'none', roles: ["Any"] },
    component: () => PageNotFound,
  },

  { path: '/', redirect: '/login' },
  { path: '/:pathMatch(.*)*', redirect: '/404' },
]

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
