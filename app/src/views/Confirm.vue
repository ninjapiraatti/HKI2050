<template>
	<div v-if='confirmed' class='container mt-5'>
		<div class='card shadow'>
			<div class='card-header'>
				<h1 class='h3 mb-0'>Account confirmed</h1>
			</div>
			<div class='card-body'>
				<p class='mb-0'>Your account has been confirmed. You can now <router-link :to='{ name: "login" }'>log in</router-link>.</p>
			</div>
		</div>
	</div>
</template>

<script>
import FormResetPassword from '@forms/FormResetPassword.vue'
import { useRoute, useRouter} from 'vue-router'
import { onMounted, inject } from 'vue'
export default {
	name: 'Confirm',
	setup() {
		const modal = inject('modal')
		const route = useRoute()
		const router = useRouter()
		let confirmed = false

		onMounted(async() => {
			const data = route.query

			if (data.type == 'reset') {
				resetPassword(data)
			} else {
				confirmAccount(data)
			}
		})

		async function resetPassword(data) {
			await modal({
				title: 'Enter new password',
				component: FormResetPassword,
				props: data,
				backdrop: 'static',
			})

			router.push({ name: 'login' })
		}

		async function confirmAccount(data) {
			confirmed = await api.users.registration.confirm(data)

			const message = confirmed ? {
				type: 'success',
				title: 'Account confirmed',
			} : {
				type: 'error',
				title: 'Account confirmation failed',
			}

			flashMessage.show({
				...message,
				time: 5000,
			})

			if (!confirmed) router.replace({
				name: 'error',
				params: {
					title: 'Account confirmation failed',
					message: 'If your account is already confirmed you can try logging in.',
				},
			})

			if (data.reset_request_id) {
				confirmed = false // Hide confirmation info in the backround
				resetPassword({
					id: data.reset_request_id,
					email: data.email,
					type: 'reset',
				})
			}
		}
	}
}
</script>
