<template>
	<div v-if="form">
		<VForm @submit='onSubmit' v-slot='{ errors }' class='vstack gap-2'>

			<label for='email' class='form-label'>The email you used to register</label>
			<div class='input-group' :class='{ "has-validation": errors.email }'>
				<VField
					v-model='form.email'
					rules='required|email'
					type='email'
					id='email'
					name='email'
					label='Email'
					aria-label='Email'
					class='form-control'
					:class='{ "is-invalid": errors.email }'
				/>
				<button type='submit' :disabled='sending' class='btn btn-primary gradient'>{{ submitLabel }}</button>
				<ErrorMessage name='email' class='invalid-feedback shake' />
			</div>

		</VForm>

		<div class='mt-label'>
			Suddenly remember? <router-link :to='{ name: "login" }'>Log in</router-link>
		</div>
	</div>
</template>

<script>
import { flashMessage } from '@smartweb/vue-flash-message';
import { inject, computed, onMounted } from 'vue'
export default {
	name: 'FormForgotPassword',

	setup(_, {emit}) {
		const api = inject('api')
		let sending = false
		let form = {email: ''}

		const submitLabel = computed(() => {
			return sending ? 'Requesting' : 'Request'
		})

		async function onSubmit() {
			sending = true
			const success = await api.users.password.requestReset(form)

			const message = success ? {
				type: 'success',
				title: 'Reset request sent, check your email.',
			} : {
				type: 'error',
				title: 'Reset request failed.',
			}

			flashMessage.show({
				...message,
				time: 5000,
			})

			if (success) emit('success', success)

			sending = false
		}

		return {
			sending,
			submitLabel,
			form,
			onSubmit
		}
	}
}
</script>
