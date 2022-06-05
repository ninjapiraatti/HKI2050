<template>
	<div class="container">
		<div class="card shadow" :class='colorScheme.card'>
			<div class='card-header'>
				<h1 class="h3 mb-0">{{ title }}</h1>
			</div>
		</div>
		<VForm @submit='onSubmit' v-slot='{ errors }' class='vstack gap-2'>
			<div>
				<label for='title' class='form-label'>Title</label>
				<VField
					v-model='title'
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
					v-model='ingress'
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
					v-model='body'
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
					v-model="character_id"
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
import { inject, ref, computed, onMounted, watch, reactive, toRefs } from 'vue'
import { marked } from 'marked'
import { useRoute } from 'vue-router'
import TagTool from '@components/TagTool.vue'
export default {
	name: 'Editor',
	components: {
		TagTool,
	},
	setup(_, {emit}) {
		const store = inject('store')
		const api = inject('api')
		const route = useRoute()
		const colorScheme = inject('colorScheme')
		let sending = false
		let form = reactive({
			id: '',
			character_id: '',
			title: '',
			ingress: '',
			body: '',
			user_id: store.state.loggeduser.id,
		})
		let characters = ref([])

		onMounted(async () => {
			getCharacters()
			if (route.params.id) {
				getArticle()
			}
		})

		async function onSubmit() {
			sending = true
			let article = await api.users.articles.save(form)
			if (article) emit('success', article)

			sending = false
		}

		async function getArticle() {
			console.log(route.params.id)
			let article = await api.articles.get({id: route.params.id})
			if (article) {
				form = article[0]
			}
			console.log(form.title)
		}

		async function getCharacters() {
			characters.value = await api.users.characters.get({ user_id: store.state.loggeduser.id })
			console.log(characters.value)
		}

		const submitLabel = computed(() => {
			return sending ? 'Saving' : 'Save'
		})

		const compiledMarkdown = computed(() => {
			return marked(form.body, { sanitize: true })
		})

		return {
			store,
			colorScheme,
			sending,
			...toRefs(form),
			submitLabel,
			characters,
			getCharacters,
			getArticle,
			onSubmit,
			compiledMarkdown,
		}
	},
}
</script>
