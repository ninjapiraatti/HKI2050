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
					as='textarea'
					rows='10'
					id='body'
					name='body'
					label='body'
					aria-label='body'
					class='form-control'
					:class='{ "is-invalid": errors.body }'
				/>
				<ErrorMessage name='body' class='invalid-feedback shake' />
			</div>
			<div>
				<label for='author' class='form-label'>Author</label>
				<VField
					v-model="form.character_id"
					rules='required'
					as="select"
					name="author"
					class="form-select"
					:class='{ "is-invalid": errors.author }'
					id="character_id"
					aria-label="Pick author character"
				>
					<option :value="null" disabled selected>Pick author character</option>
					<option v-for="character in characters" :key="character" :value="character.id">
						{{ character.name }}
					</option>
				</VField>
				<ErrorMessage name='author' class='invalid-feedback shake' />
			</div>
			<TagTool></TagTool>
			<div class='mt-label'>
				<button type='submit' :disabled='sending' class='btn btn-primary gradient float-end'>{{ submitLabel }}</button>
			</div>
		</VForm>
		<div v-html="compiledMarkdown">
		</div>
	</div>
</template>

<script>
import { inject, ref, computed, onMounted, watch } from 'vue'
import { marked } from 'marked'
import TagTool from '@components/TagTool.vue'
export default {
	name: 'Editor',
	props: {
		id: {
			type: String,
			default: undefined,
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
			default: "Say your piece.",
		},
	},
	components: {
		TagTool,
	},
	setup(props, {emit}) {
		const store = inject('store')
		const api = inject('api')
		const colorScheme = inject('colorScheme')
		let sending = false
		let form = ref({ ...props, user_id: store.state.loggeduser.id })
		let characters = ref([])

		onMounted(async () => {
			getCharacters()
		})

		async function onSubmit() {
			console.log("lol")
			sending = true
			let article = await api.users.articles.save(form)
			if (article) emit('success', article)

			sending = false
		}

		async function getCharacters() {
			characters.value = await api.users.characters.get({ user_id: store.state.loggeduser.id })
			console.log(characters.value)
		}

		const submitLabel = computed(() => {
			return sending ? 'Saving' : 'Saveee'
		})

    const compiledMarkdown = computed(() => {
      return marked(form.value.body, { sanitize: true })
    })

		return {
			store,
			colorScheme,
			sending,
			form,
			submitLabel,
			characters,
			getCharacters,
			onSubmit,
			compiledMarkdown,
		}
	},
}
</script>
