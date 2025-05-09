export const allRoutes = [
  {
    path: '/login',
    name: "Login",
    component: () => import('../pages/Login.vue'),
    meta: { title: "Login", layout: 'none', roles: ["Any"] },
  },

  {
    path: '/404',
    name: "404",
    component: () => import('../pages/404.vue'),
    meta: { title: "Page not found", layout: 'none', roles: ["Any"] },
  },

  {
    path: '/dashboard',
    name: "Dashboard",
    component: () => import('../layout/Dashboard.vue'),
    meta: { title: "Dashboard", layout: 'overview', roles: ["viewer", "editor", "admin"] },
  },

  {
    path: '/accounts',
    name: "Accounts",
    component: () => import('../layout/Accounts.vue'),
    meta: { title: "Accounts", layout: 'overview', roles: ["admin"] },
  },

  {
    path: '/settings',
    name: "Settings",
    // component: () => import('../views/Settings.vue'),
    meta: { title: "Settings", layout: 'overview', roles: ["Any"] },
    children: [
      {
        path: 'profile',
        name: "Profile",
        component: () => import('../layout/settings/Profile.vue'),
        meta: { title: "Settings / Profile", layout: 'overview', roles: ["editor"] },
      },
      {
        path: 'security',
        name: "Security",
        component: () => import('../layout/settings/Security.vue'),
        meta: { title: "Settings / Security", layout: 'overview', roles: ["admin"] },
      },
    ],
  },

  { path: '/', redirect: '/login' },
  { path: '/:pathMatch(.*)*', redirect: '/404' },
]
