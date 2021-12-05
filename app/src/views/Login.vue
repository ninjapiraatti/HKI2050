<template>

</template>

<script>
import { inject, onMounted } from 'vue'
import FormLogin from '@forms/FormLogin.vue'
//import modal from '@root/plugins/vue-modal'
export default {
	name: 'Login',
	setup() {
		const modal = inject('modal')
		onMounted(async() => {
			const success = await modal({
				title: 'Log in',
				component: FormLogin,
				backdrop: 'static',
			})
			if (success) {
				console.log("Logged in")
				this.$router.replace(this.$route.query.redirect || { name: 'home' })
			}
		})

		return {
			modal,
		}
	},
}
</script>
