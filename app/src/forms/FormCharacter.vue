<template>
	<VForm @submit='onSubmit' v-slot='{ errors }' class='vstack gap-2'>
		<div>
			<label for='label' class='form-label'>Name</label>
			<VField
				v-model='form.name'
				rules='required'
				type='text'
				id='label'
				name='label'
				label='Name'
				aria-label='Name'
				class='form-control'
				:class='{ "is-invalid": errors.name }'
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
import { inject, computed, onMounted } from 'vue'
export default {
	name: 'FormCharacter',
	props: {
		id: {
			type: String,
			default: undefined,
		},
		user_id: {
			type: String,
			required: true,
		},
		name: {
			type: String,
			default: undefined,
		},
		description: {
			type: String,
			default: undefined,
		},
	},

	setup(props, {emit}) {
		const store = inject('store')
		const api = inject('api')
		let sending = false
		let form = { ...props }

		async function onSubmit() {
			sending = true
			let character = await api.users.characters.save(form)
			if (character) emit('success', character)

			sending = false
		}

		const submitLabel = computed(() => {
			return sending ? 'Saving' : 'Save'
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
