<template>
	<div>
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
import { inject } from 'vue'
	export default {
	name: 'FormRegister',
	setup() {
		const store = inject('store')
		return {
			store
		}
	},
	data() {
		return {
			sending: false,
			form: {
				email: '',
				username: '',
				password_plain: '',
			}
		}
	},

	computed: {
		isAdmin() {
			return this.store.state.loggeduser && this.store.state.loggeduser.isadmin
		},

		submitLabel() {
			return this.isAdmin
				? this.sending ? 'Inviting' : 'Invite'
				: this.sending ? 'Registering' : 'Register'
		},
	},

	methods: {
		async onSubmit() {
			this.sending = true

			const success = await this.$api.users.registration.invite(this.form)

			if (success) {
				this.$emit('success', success)

				this.$flashMessage.show({
					type: 'success',
					title: 'Invitation sent',
					time: 5000,
				})
			}

			this.sending = false
		}
	}
}
</script>
