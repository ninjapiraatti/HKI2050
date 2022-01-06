<template>
	<VForm v-if="form" @submit='onSubmit' v-slot='{ errors }' class='vstack gap-2'>
		<div>
			<label for='password' class='form-label'>Password</label>
			<VField
				v-model='form.password'
				rules='required'
				type='password'
				id='password'
				name='password'
				label='Password'
				aria-label='Password'
				class='form-control'
				:class='{ "is-invalid": errors.password }'
			/>
			<ErrorMessage name='password' class='invalid-feedback shake' />
		</div>

		<div>
			<label for='password_confirmation' class='form-label'>Repeat password</label>
			<VField
				rules='confirmed:@password'
				type='password'
				id='password_confirmation'
				name='password_confirmation'
				label='Password confirmation'
				aria-label='Password confirmation'
				class='form-control'
				:class='{ "is-invalid": errors.password_confirmation }'
			/>
			<ErrorMessage name='password_confirmation' class='invalid-feedback shake' />
		</div>

		<div class='mt-label'>
			<button type='submit' :disabled='sending' class='btn btn-primary gradient float-end'>{{ submitLabel }}</button>
		</div>
	</VForm>
</template>

<script>
import { flashMessage } from '@smartweb/vue-flash-message';
import { inject, computed, onMounted } from 'vue'
export default {
	name: 'FormResetPassword',

	props: {
		id: {
			type: String,
			required: true,
		},
		email: {
			type: String,
			required: true,
		},
	},

	setup(props, {emit}) {
		const store = inject('store')
		const api = inject('api')
		let sending = false
		let form = { 
			...props,
			password: '' 
		}

		async function onSubmit() {
			sending = true

			for (const prop in form) if (form[prop] == '') form[prop] = undefined
			const success = await api.users.password.save(form)
			
			const message = success ? {
				type: 'success',
				title: 'Password changed',
			} : {
				type: 'error',
				title: 'Updating password failed',
			}

			flashMessage.show({
				...message,
				time: 5000,
			})
			
			if (success) emit('success', success)

			sending = false
		}

		const submitLabel = computed(() => {
			return sending ? 'Changing' : 'Change'
		})

		onMounted(() => {
			console.log(props)
		})

		return {
			store,
			sending,
			submitLabel,
			form,
			onSubmit
		}
	},
}
</script>
