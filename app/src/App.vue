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
import { provide, inject } from 'vue'
import store from '@root/store'
//import modal from '@root/plugins/vue-modal'
import TheHeader from '@components/TheHeader.vue'
import FormLogin from '@forms/FormLogin.vue'
export default {
	setup() {
		provide('store', store)
		//provide('modal', modal)

		return {
			store,
			//modal,
		}
	},
	name: 'App',
	components: {
		TheHeader,
	},
}
</script>
