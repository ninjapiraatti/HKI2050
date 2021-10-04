<template>
	<div class="container">
		<div class="row gx-4">
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
			<div class="col-md-8">
				Lol
			</div>
		</div>
	</div>
</template>

<script>
	export default {
		name: 'UserProfile',

		data() {
			return {
				user: {},
				characters: [],
			}
		},

		async mounted() {
			if (this.$store.state.loggeduser.isadmin && !this.$store.state.projects.length) {
				console.log("lol")
			}

			await Promise.all([
				this.getUser(),
				this.getCharacters(),
			])
		},

		methods: {
			async getUser() {
				const promises = [ this.$api.users.get({ id: this.$route.params.id }) ]
				if (!this.$store.state.skillLevels.length) promises.push(this.$store.dispatch('getSkillLevels'))

				const [ user ] = await Promise.all(promises)

				this.user = user

				this.user.skills.forEach(skill => {
					skill.levelLabel = this.$store.state.skillLevels.find(({ id }) => id == skill.skillscopelevel_id).label
				})
			},

			async editUser(props = {}) {
				const result = await this.$modal({
					title: 'Edit user info',
					component: FormUserInfo,
					props,
				})

				if (result) this.getUser()
			},

			async getCharacters() {
				this.characters = await this.$api.users.characters.get({ id: this.$route.params.id })
			},

			async confirmDelete(type, data) {
				const success = await this.$confirm.delete(type, data)
				
				if (success) {
					switch (type) {
						case 'user':
							if (data.id == this.$store.state.loggeduser.id) await this.$api.users.log.out()
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
