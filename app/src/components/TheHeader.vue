<template>
	<div>
		<button v-on:click="logOut()">Log out</button>
	</div>
</template>

<script>
import { inject } from 'vue'
export default {
	name: 'TheHeader',
	setup() {
		const store = inject('store')
		return {
			store
		}
	},
	data() {
		return {
			loggedUser: this.store.state.loggeduser // Here loggedUser will result to menu rendering vs. not if you use the state directly
		}
	},

	methods: {
		logOut() {
			const success = this.store.methods.logout()
			console.log(success)
			if (success) {
				this.$flashMessage.show({
					type: 'success',
					title: 'Successfully logged out',
					time: 5000,
				})
				this.$router.push({ name: 'login' })
			}
		},
	},
}
</script>
