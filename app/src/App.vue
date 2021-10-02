<template>
	<div>
		<TheHeader v-on:loggedout="logOut" v-if="store.state.counter"/>
		<FlashMessage position="right top" />
		<TheModal />
		<main id='main'>
			<router-view />
		</main>
	</div>
</template>

<script>
import { provide } from 'vue'
import store from '@root/store'
import TheHeader from '@components/TheHeader.vue' 
export default {
	setup() {
		provide('store', store)
		return {
			store
		}
	},
	name: 'App',
	components: {
		TheHeader
	},

	methods: {
		logOut() {
			this.$router.push({ name: 'login' })
		},
	},
}
</script>
