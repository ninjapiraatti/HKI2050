<template>
	<div v-if="articleProp.user_id" class="container">
		<FormArticle :article='articleProp' />
	</div>
</template>

<script>
import FormArticle from '@forms/FormArticle.vue'
import { useRoute } from 'vue-router'
import { inject, ref, onMounted, computed } from 'vue'
export default {
		name: "Editor",
		setup() {
			const api = inject('api')
			const route = useRoute()
			const store = inject('store')

			let articleObject = ref({})

			async function getArticle() {
				console.log(route.params.id)
				let article = await api.articles.get({id: route.params.id})
				if (article) {
					articleObject.value = article[0]
				}
			}

			onMounted(async () => {
				if (route.params.id) {
					getArticle()
				}
			})

			const articleProp = computed(() => {
				return articleObject.value
			})

			return {
				articleObject,
				articleProp,
				getArticle,
			};
		},
		components: { FormArticle }
}
</script>
