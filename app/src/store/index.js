import { reactive } from 'vue'
import { api } from '@root/api.js'

const state = reactive({
	loggeduser: JSON.parse(localStorage.getItem('user')),
	colorScheme: getComputedStyle(document.documentElement).getPropertyValue('--color-scheme').trim(),
	tags: [],
})

const methods = {

	async logout() {
		try {
			await api.users.log.out()
			await this.setUser(null)
		} catch (error) {
			console.warn(`Logout failed: ${error.message}`)
			return false
		}
		return true
	},

	async login(data) {
		try {
			const userId = await api.users.log.in(data)
			if (userId) await this.setUser(userId)
		} catch (error) {
			console.warn(`Login failed: ${error.message}`)
			return false
		}
		return true
	},

	async setUser(data) {
		if (typeof data == 'string') {
			try {
				const [ user ] = await Promise.all([
					api.users.get({ id: data }),
				])

				data = user
			} catch (error) {
				data = null
			}
		}
		state.loggeduser = data
		if (data) {
			localStorage.setItem('user', JSON.stringify(data))
		} else {
			localStorage.removeItem('user')
		}
	},
}

export default {
	state,
	methods
}