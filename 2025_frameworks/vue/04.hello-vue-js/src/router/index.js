import { createRouter, createWebHistory } from 'vue-router'
import Login from '../pages/Login.vue'

const routes = [
  {
    path: '/login', name: 'Login', component: Login,
    meta: { title: 'Vue - Login' },
  },
  {
    path: '/hello', name: 'Hello',
    component: () => import('../pages/Hello.vue'),
    meta: { title: 'Vue - Hello' },
  },
  {
    path: '/:pathMatch(.*)*',
    redirect: '/login',
  },
  /*
  {
    path: '/:pathMatch(.*)*', name: 'NotFound',
    component: () => import('../components/NotFound.vue'),
  }
  */
]

const router = createRouter({
  history: createWebHistory(import.meta.env.VITE_BASE_PATH),
  routes
})

router.beforeEach((to, _from, next) => {
  document.title = String(to.meta.title) || 'Page Not Found';
  next();
})

export default router;
