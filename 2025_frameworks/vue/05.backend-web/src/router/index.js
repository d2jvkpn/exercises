import { createRouter, createWebHistory } from 'vue-router'

const routes = [
  {
    path: '/login',
    component: () => import('../pages/Login.vue'),
    meta: { title: "Login", layout: 'none' },
  },

  {
    path: '/hello',
    component: () => import('../pages/Hello.vue'),
    meta: { title: "Hello", layout: 'none' },
  },

  {
    path: '/dashboard',
    component: () => import('../layout/Dashboard.vue'),
    meta: { title: "Dashboard", layout: 'admin' },
  },

  {
    path: '/accounts',
    component: () => import('../layout/Accounts.vue'),
    meta: { title: "Accounts", layout: 'admin' },
  },

  {
    path: '/settings',
    // component: () => import('../views/Settings.vue'),
    children: [
      {
        path: 'profile',
        component: () => import('../layout/settings/Profile.vue'),
        meta: { title: "Settings / Profile", layout: 'admin' },
      },
      {
        path: 'security',
        component: () => import('../layout/settings/Security.vue'),
        meta: { title: "Settings / Security", layout: 'admin' },
      },
    ],
  },

  { path: '/', redirect: '/login' },
  { path: '/:pathMatch(.*)*', redirect: '/dashboard' },
]

//export default createRouter({
//  history: createWebHistory(),
//  routes,
//})

const router = createRouter({
  history: createWebHistory(import.meta.env.VITE_BASE_PATH),
  routes,
})

router.beforeEach((to, from, next) => {
  document.title = String(to.meta.title) || 'Page Not Found';

  const isLoggedIn = !!localStorage.getItem('token');

  if (to.meta.layout === 'admin' && !isLoggedIn) {
    next('/login')
  } else {
    next()
  }
})

export default router
