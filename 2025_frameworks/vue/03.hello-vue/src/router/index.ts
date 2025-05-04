import { createRouter, createWebHistory } from 'vue-router'
import Login from '../components/Login.vue'

const routes = [
  {
    path: '/login',
    name: 'Login',
    component: Login,
    meta: { title: 'Login' },
  },
  {
    path: '/about',
    name: 'About',
    component: () => import('../components/HelloWorld.vue'),
    meta: { title: 'About' },
  },
  {
    path: '/:pathMatch(.*)*',
    redirect: '/login',
  },
  /*
  {
    path: '/:pathMatch(.*)*',
    name: 'NotFound',
    component: () => import('../components/NotFound.vue'),
  }
  */
]

const router = createRouter({
  history: createWebHistory(import.meta.env.VITE_BASE_PATH),
  routes
})

router.beforeEach((to, from, next) => {
  document.title = to.meta.title || 'Page Not Found';
  next()
})

export default router;
