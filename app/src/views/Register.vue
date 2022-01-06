<template>
	<div v-if='registered' class='container mt-5'>
		<div class='card shadow' :class='colorScheme.card'>
			<div class='card-header'>
				<h1 class='h3 mb-0'>Invitation sent</h1>
			</div>
			<div class='card-body'>
				<p class='mb-0'>Please check your email and follow the confirmation link to activate your account.</p>
			</div>
		</div>
	</div>
</template>

<script>
import FormRegister from '@forms/FormRegister.vue'
import { inject, ref, onMounted } from 'vue'
export default {
	name: 'Register',
	setup() {
		const modal = inject('modal')
		const colorScheme = inject('colorScheme')
		let registered = ref(false)

		onMounted(async() => {
			registered.value = await modal({
				title: 'Sign up',
				component: FormRegister,
				backdrop: 'static',
			})
		})

		return {
			colorScheme,
			registered
		}
	},
}
</script>
