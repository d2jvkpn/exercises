import { createRouter, createWebHistory } from 'vue-router'

const routes = [
  {
    path: '/login',
    component: () => import('../components/Login.vue'),
    meta: { title: "Login", layout: 'none' },
  },
  {
    path: '/dashboard',
    component: () => import('../views/Dashboard.vue'),
    meta: { title: "Dashboard", layout: 'admin' },
  },
  {
    path: '/accounts',
    component: () => import('../views/Accounts.vue'),
    meta: { title: "Accounts", layout: 'admin' },
  },
  {
    path: '/settings',
    // component: () => import('../views/Settings.vue'),
    children: [
      {
        path: 'profile',
        component: () => import('../views/settings/Profile.vue'),
        meta: { title: "Settings / Profile", layout: 'admin' },
      },
      {
        path: 'security',
        component: () => import('../views/settings/Security.vue'),
        meta: { title: "Settings / Security", layout: 'admin' },
      },
    ],
  },

  {
    path: '/',
    redirect: '/login',
  }
]

//export default createRouter({
//  history: createWebHistory(),
//  routes,
//})

const router = createRouter({
  // history: createWebHistory(),
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
