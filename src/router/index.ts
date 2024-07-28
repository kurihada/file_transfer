import { createRouter, createWebHistory } from 'vue-router'

const routes = [
    {
        path: '/',
        redirect: '/MainWindows/Note',
    },
    {
        path: '/MainWindows/Note',
        component: () => import('../views/NoteBookEdit.vue'),
    },
    {
        path: '/Windows/About',
        component: () => import('../views/AppAbout.vue'),
    },
    {
        path: '/Loading',
        component: () => import('../views/AppLoading.vue'),
    },
]

const router = createRouter({
    history: createWebHistory(),
    routes,
})

export default router
