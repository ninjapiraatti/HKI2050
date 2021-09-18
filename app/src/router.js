import { createRouter, createWebHistory } from 'vue-router'
import { Modal } from 'bootstrap'

import Home from './views/Home.vue'

const error = (to, props = {}) => ({
	name: 'error',
	params: {
		...props,
		pathMatch: to.path.split('/').slice(1),
	}
})

const needLogin = to => state.loggeduser ? true : { name: 'login', query: { redirect: to.fullPath } }

const needAdmin = to => state.loggeduser.isadmin ? true : error(to)

const needAdminOrSelf = to => state.loggeduser.isadmin || to.params.id == state.loggeduser.id ? true : error(to)

const router = createRouter({
	routes: [
		//{ path: '/', name: 'home', redirect: () => ({ name: state.loggeduser && state.loggeduser.isadmin ? 'admin-home' : 'user-home' }) },
		{ path: '/', name: 'home', redirect: () => ('page-home') },
		{ path: '/app', component: Home, name: 'page-home' },
		//{ path: '/app/user/:id', component: UserProfile, name: 'user', beforeEnter: [needLogin, needAdminOrSelf] },
		//{ path: '/app/admin', component: Admin, beforeEnter: [needLogin, needAdmin], children: [
		//	{ path: 'projects', component: AdminProjects, name: 'admin-projects' },
		//] },
		{ path: '/:pathMatch(.*)*', component: Error, name: 'error', props: true },
	],
	history: createWebHistory(),
	linkActiveClass: 'active',
	linkExactActiveClass: ''
})

export default router
