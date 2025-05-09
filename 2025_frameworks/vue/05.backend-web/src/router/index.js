import { createRouter, createWebHistory } from 'vue-router'

export const allRoutes = [
  {
    path: '/login',
    name: "Login",
    component: () => import('@/pages/Login.vue'),
    meta: { title: "Login", layout: 'none', roles: ["Any"] },
  },

  {
    path: '/dashboard',
    name: "Dashboard",
    component: () => import('@/layout/Dashboard.vue'),
    meta: { title: "Dashboard", layout: 'overview', roles: ["editor", "admin"] },
  },

  {
    path: '/accounts',
    name: "Accounts",
    component: () => import('@/layout/Accounts.vue'),
    meta: { title: "Accounts", layout: 'overview', roles: ["admin"] },
  },

  {
    path: '/settings',
    name: "Settings",
    // component: () => import('@/views/Settings.vue'),
    meta: { title: "Settings", layout: 'overview', roles: ["Any"] },
    children: [
      {
        path: 'profile',
        name: "Profile",
        component: () => import('@/layout/settings/Profile.vue'),
        meta: { title: "Settings / Profile", layout: 'overview', roles: ["editor"] },
      },
      {
        path: 'security',
        name: "Security",
        component: () => import('@/layout/settings/Security.vue'),
        meta: { title: "Settings / Security", layout: 'overview', roles: ["admin"] },
      },
    ],
  },

  {
    path: '/page-not-found',
    name: "PageNotFound",
    component: () => import('@/pages/PageNotFound.vue'),
    meta: { title: "Page not found", layout: 'none', roles: ["Any"] },
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
