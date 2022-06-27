<template>
	<div class="card shadow" :class='colorScheme.card'>
		<div class='card-header'>
			<h1 class="h3 mb-0">{{ form.title }}</h1>
			{{ form }}
		</div>
		<VForm v-if='showFormBool == true' @submit='onSubmit' v-slot='{ errors }' class='vstack gap-2'>
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
			<div v-if='characters && characters.length'>
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
					<option v-if='!form.character_id' :value="null" disabled>Pick author character</option>
					<option v-for="character in characters" :key="character.id" :value="character.id">
						{{ character.name }}
					</option>
				</VField>
				<ErrorMessage name='author' class='invalid-feedback shake' />
			</div>
			<TagTool @tagsUpdated='updateTags'></TagTool>
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
import { flashMessage } from '@smartweb/vue-flash-message';
import TagTool from '@components/TagTool.vue'
export default {
	name: 'FormArticle',
	props: {
		article: {
			type: Object,
			default: undefined,
		},
	},
	components: {
		TagTool,
	},
	setup(props, {emit}) {
		const store = inject('store')
		const api = inject('api')
		const route = useRoute()
		const colorScheme = inject('colorScheme')
		let sending = false
		let form = reactive({
			id: props.article.id || '',
			character_id: props.article.character_id || '',
			title: props.article.title || '',
			ingress: props.article.ingress || '',
			body: props.article.body || '',
			user_id: store.state.loggeduser.id, // This won't do in long run
		})
		let characters = ref([])
		let articles = ref([])
		let showFormBool = ref(false)

		async function onSubmit() {
			sending = true
			let article = await api.users.articles.save(form)
			if (article) {
				flashMessage.show({
					type: 'success',
					title: 'Article saved.',
					time: 500,
				})
			}

			sending = false
		}

		async function updateTags(tags) {
			console.log(tags)
			//form.tags = tags
		}

		const showForm = computed(async() => {
			console.log("Fetching user articles")
			articles.value = await api.users.articles.get({ user_id: store.state.loggeduser.id })
			if (articles.value.length) {
				showFormBool.value = true
				return true
			}
			if (!route.params.id) {
				showFormBool.value = true
				return true
			}
			showFormBool.value = false
			return false
		})

		const submitLabel = computed(() => {
			return sending ? 'Saving' : 'Save'
		})

		const compiledMarkdown = computed(() => {
			if (form.body) {
				return marked(form.body, { sanitize: true })
			}
		})

		async function getCharacters() {
			characters.value = await api.users.characters.get({ user_id: store.state.loggeduser.id })
		}

		onMounted(async () => {
			getCharacters()
			showForm.value
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
			showForm,
			showFormBool,
		}
	},
}
</script>
