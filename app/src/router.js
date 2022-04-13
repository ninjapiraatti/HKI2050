import { createRouter, createWebHistory } from 'vue-router'
import state from '@store/index.js'
import { Modal } from 'bootstrap'
import Login from '@views/Login.vue'
import UserHome from '@views/UserHome.vue'
import UserProfile from '@views/UserProfile.vue'
import ForgotPassword from '@views/ForgotPassword.vue'
import Register from '@views/Register.vue'
import Confirm from '@views/Confirm.vue'
import HKIbook from '@views/HKIbook.vue'
import Article from '@views/Article.vue'
import Error from '@views/Error.vue'

import Home from './views/Home.vue'

const error = (to, props = {}) => ({
	name: 'error',
	params: {
		...props,
		pathMatch: to.path.split('/').slice(1),
	}
})

const needLogin = to => state.state.loggeduser ? true : { name: 'login', query: { redirect: to.fullPath } }

const needAdmin = to => state.state.loggeduser.isadmin ? true : error(to)

const needAdminOrSelf = to => state.state.loggeduser.isadmin || to.params.id == state.state.loggeduser.id ? true : error(to)

const router = createRouter({
	routes: [
		{ path: '/', name: 'home', redirect: () => ({ name: state.state.loggeduser && state.state.loggeduser.isadmin ? 'admin-home' : 'user-home' }) },
		{ path: '/app/confirm', component: Confirm, name: 'confirm' },
		{ path: '/app/forgotpassword', component: ForgotPassword, name: 'forgot-password' },
		{ path: '/app/register', component: Register, name: 'register' },
		{ path: '/app/user', component: UserHome, name: 'user-home', beforeEnter: [needLogin] },
		{ path: '/', name: 'home', redirect: () => ({name: 'page-home'}) },
		{ path: '/app/', component: Home, name: 'page-home' },
		{ path: '/app/login', component: Login, name: 'login' },
		{ path: '/app/hkibook', component: HKIbook, name: 'hkibook' },
		{ path: '/app/hkibook/:id', component: Article, name: 'article-open' },
		{ path: '/app/hkibook/new', component: Article, name: 'article-new', beforeEnter: [needLogin, needAdmin] },
		{ path: '/app/user/:id', component: UserProfile, name: 'user', beforeEnter: [needLogin, needAdminOrSelf] },
		//{ path: '/app/admin', component: Admin, beforeEnter: [needLogin, needAdmin], children: [
		//	{ path: 'projects', component: AdminProjects, name: 'admin-projects' },
		//] },
		{ path: '/:pathMatch(.*)*', component: Error, name: 'error', props: true },
	],
	history: createWebHistory(),
	linkActiveClass: 'active',
	linkExactActiveClass: ''
})

// Close modal before navigating
router.beforeEach((to, from, next) => {
	const modal = document.querySelector('.modal.show')
	if (!modal) {
		next()
	} else {
		modal.addEventListener('hidden.bs.modal', () => next())
		Modal.getInstance(modal).hide()
	}
})

export default router
