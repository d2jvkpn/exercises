import { createRouter, createWebHistory } from 'vue-router'
import PageNotFound from '@/pages/PageNotFound.vue'

export const allRoutes = [
  {
    path: '/login',
    name: "Login",
    meta: { title: "App - Login", layout: 'none', roles: ["Any"] },
    component: () => import('@/pages/Login.vue'),
  },

  {
    path: '/home/dashboard',
    name: "Dashboard",
    meta: { title: "App - Dashboard", layout: 'home', roles: ["editor", "admin"] },
    component: () => import('@/layout/Dashboard.vue'),
  },

  {
    path: '/home/accounts',
    name: "Accounts",
    meta: { title: "App - Accounts", layout: 'home', roles: ["admin"] },
    component: () => import('@/layout/Accounts.vue'),
  },

  {
    path: '/home/settings',
    name: "Settings",
    meta: { title: "App - Settings", layout: 'home', roles: ["Any"] },
    // component: () => import('@/layout/settings/Settings.vue'),
    children: [
      {
        path: 'profile',
        name: "Profile",
        meta: { title: "App - Profile", layout: 'home', roles: ["editor"] },
        component: () => import('@/layout/settings/Profile.vue'),
      },
      {
        path: 'security',
        name: "Security",
        meta: { title: "App - Security", layout: 'home', roles: ["admin"] },
        component: () => import('@/layout/settings/Security.vue'),
      },
    ],
  },

  {
    path: '/page-not-found',
    name: "PageNotFound",
    meta: { title: "App - Page not found", layout: 'none', roles: ["Any"] },
    component: () => PageNotFound,
  },

  { path: '/', redirect: '/login' },
  { path: '/:pathMatch(.*)*', redirect: '/page-not-found' },
]

const router = createRouter({
  history: createWebHistory(import.meta.env.VITE_BASE_PATH),
  routes: allRoutes,
})

router.beforeEach((to, from, next) => {
  document.title = String(to.meta.title) || 'Page Not Found';

  const isLoggedIn = !!localStorage.getItem('token');

  if (to.meta.layout === 'home' && !isLoggedIn) {
    next('/login')
  } else {
    next()
  }
})

export default router
