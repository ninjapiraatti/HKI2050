<template>
	<div class="container">
		{{ user }}
		<div v-if="user" class="row gx-4">
			<div class="col-md-4">
				<div class="card shadow" :class='$colorScheme.card'>
					<div class='context'>
						<div class='card-header d-flex align-items-center'>
							<h1 class="h3 mb-0 flex-grow-1">
								{{ user.username }}
							</h1>
						</div>
						<div class='card-body'>
							<div>{{ user.email }}</div>
							<div class='context-actions hstack gap-1 justify-content-end'>
								<button class='btn btn-unstyled px-1 rounded' v-on:click="editUser(user)"><i class="bi-pencil-fill" title='Edit profile'></i></button>
								<button class='btn btn-unstyled px-1 rounded' v-on:click="confirmDelete('user', user)"><i class="bi-trash-fill" title='Delete profile'></i></button>
							</div>
						</div>
					</div>
				</div>
			</div>
			<div class="mt-4 mt-md-0 col-md-8">
				<div class="card shadow" :class='$colorScheme.card'>
					<div class='card-header'>
						<div class="d-flex flex-wrap justify-content-between align-items-center">
							<h3 class="h3 mb-0">Characters</h3>
							<button class="btn btn-primary gradient" v-on:click="editCharacter()">Add character</button>
						</div>
					</div>
					<div class='card-body'>
						<div v-if='characters && characters.length' class="table-responsive">
							<table class="table table-striped mb-0" :class='$colorScheme.table'>
								<thead>
									<tr>
										<th scope="col">Character</th>
										<th scope="col">Level</th>
										<th scope="col" class='text-center'>Years</th>
										<th scope="col" class='text-end'>Actions</th>
									</tr>
								</thead>
								<tbody>
									<tr v-for="character in characters" :key="character.id" class='context'>
										<td>{{ character.name }}</td>
										<td>{{ character.description }}</td>
										<td class='text-center'>{{ character.years }}</td>
										<td class='text-end'>
											<div class='context-actions hstack gap-1 justify-content-end'>
												<button class='btn btn-unstyled px-1 rounded' v-on:click="editCharacter(character)"><i class="bi-pencil-fill" title='Edit character'></i></button>
												<button class='btn btn-unstyled px-1 rounded' v-on:click="confirmDelete('user.character', character)"><i class="bi-trash-fill" title='Delete character'></i></button>
											</div>
										</td>
									</tr>
								</tbody>
							</table>
						</div>
						<div v-else class='fs-3 fw-light text-muted text-center p-4'>No characters</div>
					</div>
				</div>
			</div>
		</div>
	</div>
</template>

<script>
import FormUserInfo from '@forms/FormUserInfo.vue'
import FormCharacter from '@forms/FormCharacter.vue'
import { api } from '@root/api.js'
import { useRoute } from 'vue-router'
import { inject, reactive, ref, onMounted, toRefs } from 'vue'
export default {
	name: 'UserProfile',
	setup() {
		const store = inject('store')
		const modal = inject('modal')
		const route = useRoute()
		let user = reactive({
			id: "",
			username: "",
			isadmin: false,
			email: "",
		})
		let characters = ref([])

		async function getCharacters() {
			characters.value = await api.users.characters.get({ user_id: route.params.id })
		}

		async function getUser() {
			const promises = [ api.users.get({ id: route.params.id }) ]
			//if (!this.store.state.characterLevels.length) promises.push(this.store.dispatch('getcharacterLevels'))
			console.log(user)
			const [ userUpdated ] = await Promise.all(promises)
			console.log(userUpdated)
			user = userUpdated
			console.log(user)

			/*
			this.user.characters.forEach(character => {
				character.levelLabel = this.store.state.characterLevels.find(({ id }) => id == character.characterscopelevel_id).label
			})
			*/
		}

		async function editUser(props = {}) {
			const result = await modal({
				title: 'Edit user info',
				component: FormUserInfo,
				props,
			})

			if (result) getUser()
			//return result
		}


		async function editCharacter(props = {}) {
			props.user_id = user.id
			const result = await modal({
				title: props.id ? `Edit skill: ${props.name}` : 'Add skill',
				component: FormCharacter,
				props,
			})

			if (result) getCharacters()
			return result
		}

		onMounted(async() => {
			if (store.state.loggeduser.isadmin && !store.state.characters.length) {
				console.log("lol")
			}
			user = store.state.loggeduser
			await Promise.all([
				getUser(),
				getCharacters(),
			])
		})

		return {
			user,
			store,
			characters,
			getCharacters,
			editUser,
			editCharacter,
		}
	},
	methods: {

		async confirmDelete(type, data) {
			console.log(type, data)
			const success = await this.$confirm.delete(type, data)
			if (success) {
				switch (type) {
					case 'user':
						if (data.id == this.store.state.loggeduser.id) await this.$api.users.log.out()
						this.$router.push({ name: 'admin-users' })
						break

					case 'user.character':
						this.getUser()
						break
				}
			}
		}
	},
}
</script>
