import { createRouter, createWebHashHistory } from 'vue-router'

import Poc1 from './components/pages/poc_1.vue'

const routes = [
  {
    path: '/',
    redirect: '/poc1'
  },
  {
    path: '/poc1',
    name: 'PoC 1',
    component: Poc1
  },
  {
    path: '/:pathMatch(.*)',
    redirect: '/'
  }
]

const router = createRouter({
  history: createWebHashHistory(),
  routes // short for `routes: routes`
})

export default router
