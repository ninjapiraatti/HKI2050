<template>
	<VForm @submit='onSubmit' v-slot='{ errors }' class='vstack gap-2'>

		<div>
			<label for='label' class='form-label'>Name</label>
			<VField
				v-model='form.label'
				rules='required'
				type='text'
				id='label'
				name='label'
				label='Name'
				aria-label='Name'
				class='form-control'
				:class='{ "is-invalid": errors.label }'
			/>
			<ErrorMessage name='label' class='invalid-feedback shake' />
		</div>

		<div>
			<label for='description' class='form-label'>Description</label>
			<VField
				v-model='form.description'
				rules='required'
				type='text'
				id='description'
				name='description'
				label='Description'
				aria-label='Description'
				class='form-control'
				:class='{ "is-invalid": errors.description }'
			/>
			<ErrorMessage name='description' class='invalid-feedback shake' />
		</div>

		<div class='mt-label'>
			<button type='submit' :disabled='sending' class='btn btn-primary gradient float-end'>{{ submitLabel }}</button>
		</div>
	</VForm> 
</template>

<script>
	export default {
		name: 'FormCharacter',

		props: {
			id: {
				type: String,
				default: undefined,
			},
		},

		computed: {
			submitLabel() {
				return this.sending ? 'Saving' : 'Save'
			},

			categories() {
				return this.store.state.skillCategories
			},

			scopes() {
				return this.store.state.skillScopes
			},
		},

		data() {
			return {
				sending: false,
				form: { ...this.$props },
			}
		},
		
		mounted() {
			if (!this.store.state.skillCategories.length) this.$store.dispatch('getSkillCategories')
			if (!this.store.state.skillScopes.length) this.$store.dispatch('getSkillScopes')
		},

		methods: {
			async onSubmit() {
				this.sending = true

				const skill = await this.$api.skills.save(this.form)
				if (skill) this.$emit('success', skill)

				this.sending = false
			},
		},
	}
</script>
