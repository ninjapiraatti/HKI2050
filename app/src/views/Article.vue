<template>
	<div v-if="articleObject" class="container">
		<FormArticle :article='articleObject' />
	</div>
</template>

<script>
import FormArticle from '@forms/FormArticle.vue'
import { useRoute } from 'vue-router'
import { inject, ref, onMounted } from 'vue'
export default {
    name: "Editor",
    setup() {
			const api = inject('api')
			const route = useRoute()
			const store = inject('store')

			let articleObject = ref({})
			let characters = ref([])

			async function getArticle() {
				console.log(route.params.id)
				let article = await api.articles.get({id: route.params.id})
				if (article) {
					articleObject.value = article[0]
				}
			}

			async function getCharacters() {
				characters.value = await api.users.characters.get({ user_id: store.state.loggeduser.id })
				console.log(characters.value)
			}

      onMounted(async () => {
				getCharacters()
				if (route.params.id) {
					getArticle()
				}
			})

			return {
				articleObject,
				getArticle,
				getCharacters,
			};
    },
    components: { FormArticle }
}
</script>
