<template>
  <div class="card shadow" :class='colorScheme.card'>
    <div class='card-header'>
      <h1 class="h3 mb-0">{{ article.title }}</h1>
    </div>
    <VForm @submit='onSubmit' v-slot='{ errors }' class='vstack gap-2'>
      <div>
        <label for='title' class='form-label'>Title</label>
        <VField
          v-model='article.title'
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
          v-model='article.ingress'
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
          v-model='article.body'
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
          v-model="article.character_id"
          rules='required'
          as="select"
          name="author"
          class="form-select"
          :class='{ "is-invalid": errors.author }'
          id="character_id"
          aria-label="Pick author character"
        >
          <option :value="null" disabled selected>Pick author character</option>
          <option v-for="character in characters" :key="character" :value="article.character.id">
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
	name: 'FormArticle',
	props: {
    /*
		id: {
			type: String,
			default: undefined,
		},
		character_id: {
			type: String,
			required: true,
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
    */
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
			id: '',
			character_id: '',
			title: '',
			ingress: '',
			body: '',
			user_id: store.state.loggeduser.id,
		})
		//let form2 = { ...props }
    let form2 = { ...props.article }
		let characters = ref([])

		async function onSubmit() {
			sending = true
			let article = await api.users.articles.save(form)
			if (article) emit('success', article)

			sending = false
		}

		const submitLabel = computed(() => {
			return sending ? 'Saving' : 'Save'
		})

		const compiledMarkdown = computed(() => {
      if (props.article.body) {
        return marked(props.article.body, { sanitize: true })
      }
		})

    onMounted(async () => {
    })

    watch(() => {
      console.log(`article is: ` + props.article.title)
    })

		return {
			store,
			colorScheme,
			sending,
			...toRefs(form),
			form2,
			submitLabel,
			characters,
			onSubmit,
			compiledMarkdown,
		}
	},
}
</script>
