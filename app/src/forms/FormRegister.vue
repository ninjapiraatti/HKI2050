<template>
	<div v-if="form">
		<VForm @submit='onSubmit' v-slot='{ errors }' class='vstack gap-2'>
			<div>
				<label for='email' class='form-label'>Email</label>
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
				<ErrorMessage name='email' class='invalid-feedback shake' />
			</div>

			<div>
				<label for='username' class='form-label'>Username</label>
				<VField
					v-model='form.username'
					rules='required'
					type='text'
					id='username'
					name='username'
					label='Username'
					aria-label='Username'
					class='form-control'
					:class='{ "is-invalid": errors.username }'
				/>
				<ErrorMessage name='username' class='invalid-feedback shake' />
			</div>

			<div v-if='!isAdmin'>
				<label for='password_plain' class='form-label'>Password</label>
				<VField
					v-model='form.password_plain'
					rules='requiredNonAdmin'
					type='password'
					id='password_plain'
					name='password_plain'
					label='Password'
					aria-label='Password'
					class='form-control'
					:class='{ "is-invalid": errors.password_plain }'
				/>
				<ErrorMessage name='password_plain' class='invalid-feedback shake' />
			</div>

			<div v-if='!isAdmin'>
				<label for='password_confirmation' class='form-label'>Repeat password</label>
				<VField
					rules='confirmed:@password_plain'
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

			<div class='mt-label d-flex gap-3 flex-row-reverse align-items-center justify-content-between'>
				<button type='submit' :disabled='sending' class='btn btn-primary gradient align-self-start'>{{ submitLabel }}</button>
				<div v-if='!isAdmin'>Already a user? <router-link :to='{ name: "login" }'>Log in</router-link></div>
			</div>
		</VForm>
	</div>
</template>

<script>
import { flashMessage } from '@smartweb/vue-flash-message';
import { inject, computed} from 'vue'
	export default {
	name: 'FormRegister',
	setup(props, {emit}) {
		const store = inject('store')
		const api = inject('api')
		let sending = false
		let form = {
			email: '',
			username: '',
			password_plain: '',
		}

		const isAdmin = computed(() => {
			return store.state.loggeduser && store.state.loggeduser.isadmin
		})

		const submitLabel = computed(() => {
			return isAdmin
				? sending ? 'Inviting' : 'Invite'
				: sending ? 'Registering' : 'Register'
		})

		
		async function onSubmit() {
			sending = true

			const success = await api.users.registration.invite(form)
			if (success) {
				emit('success', success)
				this.$flashMessage.show({
					type: 'success',
					title: 'Invitation sent',
					time: 5000,
				})
			}
			sending = false
		}

		return {
			store,
			sending,
			form,
			submitLabel,
			onSubmit
		}
	},
}
</script>
