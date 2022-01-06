<template>
	<nav class="navbar navbar-expand-sm shadow mb-4 mb-md-5" :class='colorScheme.navbar'>
		<div class="container-fluid">
			<router-link :to='{ name: "home" }' class="navbar-brand hki-logo py-0">
				<img src="/public/assets/uusilogo04_nobg.png" alt="">
			</router-link>
			<ul class="navbar-nav">
				<li class="nav-item dropdown">
					<button id="usermenu" class='nav-link btn btn-unstyled fs-2 rounded-circle' data-bs-toggle="dropdown" aria-expanded="false">
						<i class='bi-person-circle'></i>
					</button>
					<ul class="dropdown-menu dropdown-menu-end position-absolute" aria-labelledby="usermenu">
						<template v-if='loggedUser && loggedUser.isadmin'>
							<li v-for='{ name, label } of navigation' :key='name'>
								<router-link :to='{ name }' class='dropdown-item'>{{ label }}</router-link>
							</li>
							<li><hr class='dropdown-divider'></li>
						</template>
						<li v-if="loggedUser">
							<router-link :to='{ name: "user", params: { id: loggedUser.id } }' class="dropdown-item">Profile</router-link>
						</li>
						<li v-if="loggedUser">
							<button v-on:click="logOut" class="dropdown-item">Log out</button>
						</li>
						<li v-else>
							<router-link :to='{ name: "login" }' class='dropdown-item'>Log in</router-link>
							<a @click='lol' class='dropdown-item'>Say lol</a>
						</li>
					</ul>
				</li>
			</ul>
		</div>
	</nav>
</template>

<script>
import { inject, computed, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { flashMessage } from '@smartweb/vue-flash-message';
export default {
	name: 'TheHeader',
	setup() {
		const router = useRouter()
		const store = inject('store')
		const colorScheme = inject('colorScheme')

		let loggedUser = computed(() => store.state.loggeduser)

		onMounted(() => loggedUser = store.state.loggeduser)

		function logOut() {
			const success = store.methods.logout()
			if (success) {
				flashMessage.show({
					type: 'success',
					title: 'Successfully logged out',
					time: 500,
				})
				router.push({ name: 'login' })
			}
		}

		function lol() {
			console.log("lol")
		}

		return {
			store,
			loggedUser,
			colorScheme,
			logOut,
			lol
		}
	},
}
</script>
