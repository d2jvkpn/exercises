import { createRouter, createWebHistory } from 'vue-router'
import Login from '../components/Login.vue'

const routes = [
  {
    path: '/login', name: 'Login', component: Login,
    meta: { title: 'Hello - Login' },
  },
  {
    path: '/about', name: 'About',
    component: () => import('../components/HelloWorld.vue'),
    meta: { title: 'Hello - About' },
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
