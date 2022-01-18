<template>
	<div class="container">
		<div class="card shadow" :class='colorScheme.card'>
			<div class='card-header'>
				<h1 class="h3 mb-0">{{ form.title }}</h1>
			</div>
		</div>
		<VForm @submit='onSubmit' v-slot='{ errors }' class='vstack gap-2'>
			<div>
				<label for='title' class='form-label'>Title</label>
				<VField
					v-model='form.title'
					rules='required'
					type='text'
					id='title'
					name='title'
					label='Name'
					aria-label='Name'
					class='form-control'
					:class='{ "is-invalid": errors.title }'
				/>
				<ErrorMessage name='title' class='invalid-feedback shake' />
			</div>

			<div>
				<label for='ingress' class='form-label'>Ingress</label>
				<VField
					v-model='form.ingress'
					rules='required'
					type='text'
					id='ingress'
					name='ingress'
					label='ingress'
					aria-label='ingress'
					class='form-control'
					:class='{ "is-invalid": errors.ingress }'
				/>
				<ErrorMessage name='ingress' class='invalid-feedback shake' />
			</div>

			<div>
				<label for='body' class='form-label'>Content</label>
				<VField
					v-model='form.body'
					rules='required'
					type='text'
					id='body'
					name='body'
					label='body'
					aria-label='body'
					class='form-control'
					:class='{ "is-invalid": errors.body }'
				/>
				<ErrorMessage name='body' class='invalid-feedback shake' />
			</div>

			<div class='mt-label'>
				<button type='submit' :disabled='sending' class='btn btn-primary gradient float-end'>{{ submitLabel }}</button>
			</div>
		</VForm>
	</div>
</template>

<script>
import { inject } from 'vue'
export default {
	name: 'Editor',
	props: {
		id: {
			type: String,
			default: undefined,
		},
		user_id: {
			type: String,
			required: true,
		},
		title: {
			type: String,
			default: undefined,
		},
		ingress: {
			type: String,
			default: undefined,
		},
		body: {
			type: String,
			default: undefined,
		},
	},
	setup(props, {emit}) {
		const store = inject('store')
		const colorScheme = inject('colorScheme')
		let sending = false
		let form = { ...props }
		return {
			store,
			colorScheme,
			sending,
			form,
		}
	},
}
</script>
