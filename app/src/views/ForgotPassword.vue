<template>
	<div v-if='requested' class='container mt-5'>
		<div class='card shadow' :class='$colorScheme.card'>
			<div class='card-header'>
				<h1 class='h3 mb-0'>Password reset requested</h1>
			</div>
			<div class='card-body'>
				<p class='mb-0'>Password reset requested successfully, check your email.</p>
			</div>
		</div>
	</div>
</template>

<script>
import FormForgotPassword from '@forms/FormForgotPassword.vue'
import { onMounted, inject } from 'vue'
export default {
	name: 'ForgotPassword',
	setup() {
		const modal = inject('modal')
		let requested = false

		onMounted(async() => {
			requested = await modal({
				title: 'Request a password reset',
				component: FormForgotPassword,
				backdrop: 'static',
			})
		})

		return {
			requested
		}
	},
}
</script>
