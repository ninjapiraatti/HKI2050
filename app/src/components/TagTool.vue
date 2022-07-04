<template>
	<div>
		<VForm @submit='onSubmit' v-slot='{ errors }' class='vstack gap-2'>
			<div>
				<label for='tag' class='form-label'>Tags (comma separated)</label>
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
			</div>	
			<div>
				<VField
					v-if='matchedTags.length > 0'
					v-model='matchedTags'
					name='taglist'
					as='select'
					id='taglist'
					class='form-select'
					@change="onTagChange"
				>
					<option v-for="tag in matchedTags" :key="tag" :value="tag.title">
						{{ tag.title }}
					</option>
				</VField>
			</div>
		</VForm>
		<ul>
			<li v-for="tag in chosenTags" :key="tag">{{ tag }}</li>
		</ul>
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
		let chosenTags = ref([])

		onMounted(async () => {
			getTags()
		})

		async function getTags() {
			tags = await api.tags.get()
			// console.log(tags.value)
			// tags.value = tagObjects.map(tag => tag.title)
		}

		async function onTagChange(e) {
			chosenTags.value.push(e.target.value)
			console.log("emitting")
			emit('tagsUpdated', chosenTags)
		}

		const matchedTags = computed(() => {
			if (tagInput.value.includes(',')) {
				let tagTitle = tagInput.value.split(',')[0].trim()
				console.log(chosenTags.value)
				if (!chosenTags.value.includes(tagTitle)) {
					chosenTags.value.push(tagTitle)
					emit('tagsUpdated', chosenTags)
				}
				tagInput.value = ''
				return []
			}
			if (tagInput.value.length > 1) {
				return tags.filter(tag => tag.title.toLowerCase().startsWith(tagInput.value.toLowerCase()) && !chosenTags.value.includes(tag.title))
			}
			return []
		})

		return {
			tags,
			chosenTags,
			matchedTags,
			getTags,
			onTagChange,
			tagInput,
		}
	},
}
</script>
