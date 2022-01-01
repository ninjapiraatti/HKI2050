<template>	
	<VForm @submit='onSubmit' v-slot='{ errors }' class='vstack gap-2'>

		<div class='form-check' v-if='store.state.loggeduser.isadmin'>
			<label for='isadmin' class='form-label'>Admin</label>
			<VField
				v-model='form.isadmin'
				:value='true'
				:unchecked-value='false'
				type='checkbox'
				id='isadmin'
				name='isadmin'
				class='form-check-input'
			/>
		</div>

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
				type='username'
				id='username'
				name='username'
				label='username'
				aria-label='username'
				class='form-control'
				:class='{ "is-invalid": errors.username }'
			/>
			<ErrorMessage name='username' class='invalid-feedback shake' />
		</div>

		<div class='mt-label'>
			<button type='submit' :disabled='sending' class='btn btn-primary gradient float-end'>{{ submitLabel }}</button>
		</div>
	</VForm>
</template>

<script>
	import { ref, inject } from 'vue'
	export default {
		name: 'FormUserInfo',
		setup() {
			const store = inject('store')
			return {
				store
			}
		},
		props: {	
			id: {
				type: String,
				default: undefined,
			},
			isadmin: {
				type: Boolean,
				default: false,
			},
			username: {
				type: String,
				default: undefined,
			},
			email: String,
		},

		data() {
			return {
				sending: false,
				form: { ...this.$props },
			}
		},

		computed: {
			submitLabel() {
				return this.sending ? 'Saving' : 'Save'
			},
		},

		methods: {
			async onSubmit() {
				this.sending = true

				const user = await this.$api.users.save(this.form)
				if (user) {
					this.$emit('success', user)
				}
				this.sending = false
			},
		},
	}
</script>
