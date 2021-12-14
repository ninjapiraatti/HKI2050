<template>

</template>

<script>
import { inject, onMounted } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import FormLogin from '@forms/FormLogin.vue'
//import modal from '@root/plugins/vue-modal'
export default {
	name: 'Login',
	setup() {
		const router = useRouter()
		const route = useRoute()
		const modal = inject('modal')
		onMounted(async() => {
			const success = await modal({
				title: 'Log in',
				component: FormLogin,
				backdrop: 'static',
			})
			if (success) {
				console.log(router)
				router.replace(route.query.redirect || { name: 'home' })
			}
		})

		return {
			modal,
		}
	},
}
</script>
