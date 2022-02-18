<template>
	<div>
		<VForm @submit='onSubmit' v-slot='{ errors }' class='vstack gap-2'>
			<div>
				<label for='tag' class='form-label'>Tags</label>
				<VField
					v-model="tagInput"
					rules='required'
					type='text'
					id='tag'
					name='tag'
					label='tag'
					aria-label='tag'
					class='form-control'
					:class='{ "is-invalid": errors.tags }'
				/>
				<ErrorMessage name='tag' class='invalid-feedback shake' />
				<VField
					v-if='matchedTags.length > 0'
					v-model='matchedTags'
					type='select'
					id='taglist'
					name='taglist'
					class='form-select'
					@change="onTagChange"
				>
					<option v-for="tag in matchedTags" :key="tag" :value="tag.id">
						{{ tag.id }}
					</option>
				</VField>
			</div>
		</VForm>
	</div>
</template>

<script>
import { inject, ref, onMounted, computed } from 'vue'
export default {
	name: 'TagTool',
	props: {
		contentTags: {
			type: Array,
			default: function() {
				return [];
			}
		}
	},
	setup(props, {emit}) {
		const api = inject('api')
		const colorScheme = inject('colorScheme')
		let sending = false
		let tagInput = ref('')

		//let form = ref({ ...props, user_id: store.state.loggeduser.id })
		let tags = ref([ ...props.contentTags])

		onMounted(async () => {
			getTags()
		})

		async function getTags() {
			tags = await api.tags.get()
			// console.log(tags.value)
			// tags.value = tagObjects.map(tag => tag.title)
			emit('tags', tags)
		}

		async function onTagChange(e) {
			console.log(e.target.value)
			tagInput.value = e.target.value
		}

		const matchedTags = computed(() => {
			if (tagInput.value.length > 1) {
				console.log(tagInput.value)
				console.log()
				return tags.filter(tag => tag.title.toLowerCase().startsWith(tagInput.value.toLowerCase()))
			}
			return []
		})

		return {
			tags,
			matchedTags,
			getTags,
			onTagChange,
			tagInput,
		}
	},
}
</script>
