<template>
	<div class="container">
		<div class="card shadow" :class='colorScheme.card'>
			<div class='card-header'>
				<h1 class="h3 mb-0">Stories from Helsinki</h1>
			</div>
			<ul>
				<li v-for="article in articles" :key="article.id">
					<router-link :to='{ name: "article-open", params: { id: article.id } }'>{{ article.title }}</router-link>
				</li>
			</ul>
		</div>
	</div>
</template>

<script>
import { inject, ref, onMounted } from 'vue'
export default {
	name: 'HKIbook',
	setup() {
		const store = inject('store')
		const colorScheme = inject('colorScheme')
		const api = inject('api')

		let articles = ref([])

		async function getArticles() {
			articles.value = await api.articles.get()
		}

		onMounted(async () => {
			getArticles()
		})

		return {
			store,
			colorScheme,
			getArticles,
			articles,
		}
	},
}
</script>
