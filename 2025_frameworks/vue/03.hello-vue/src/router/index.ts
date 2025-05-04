import { createRouter, createWebHistory } from 'vue-router'
import Login from '../components/Login.vue'

const routes = [
  {
    path: import.meta.env.VITE_BASE_PATH + '/login',
    name: 'Login',
    component: Login,
    meta: {
      title: 'Login',
    }
  },
  {
    path: import.meta.env.VITE_BASE_PATH + '/about',
    name: 'About',
    component: () => import('../components/HelloWorld.vue'),
    meta: {
      title: 'About',
    }
  },
  {
    path: '/:pathMatch(.*)*',
    redirect: import.meta.env.VITE_BASE_PATH + '/login',
  },
  /*{
    path: '/:pathMatch(.*)*',
    name: 'NotFound',
    component: () => import('../components/NotFound.vue'),
  }
  */
]

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes
})

router.beforeEach((to, from, next) => {
  document.title = to.meta.title || 'Page Not Found';
  next()
})

export default router;
