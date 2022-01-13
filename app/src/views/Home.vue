<template>
	<div class="container">
		<div class="row gx-4">
			<div class="col">
				<div class="bigasslogo">
					<img src="/public/assets/uusilogo04_square_nobg.png" alt="">
				</div>
			</div>
		</div>
	</div>
</template>

<script>
import { inject } from 'vue'
export default {
	name: 'Home',
	setup() {
		const store = inject('store')
		const colorScheme = inject('colorScheme')
		return {
			store,
			colorScheme
		}
	},
}
</script>