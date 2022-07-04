<template>
	<div>
		<TheHeader />
		<FlashMessage position="right top" />
		<TheModal />
		<main id='main'>
			<router-view />
		</main>
	</div>
</template>

<script>
import { provide, onMounted } from 'vue'
import store from '@root/store'
import TheHeader from '@components/TheHeader.vue'
export default {
	name: 'App',
	components: {
		TheHeader,
	},
	setup() {
		provide('store', store)

		onMounted(() => {
			store.methods.initState()
		})

		return {
			store,
		}
	},
}
</script>
