<template>
	<div class="container">
		<div class="counter">
			{{ store.state.counter }}
			<button @click="store.methods.decrease">-</button>
			<button @click="store.methods.increase">+</button>
		</div>
		<div class="bigasslogo">
			<img src="/public/assets/uusilogo04_square_nobg.png" alt="">
		</div>
	</div>
</template>

<script>
import { ref, inject } from 'vue'
export default {
	name: 'Home',
	setup() {
		const store = inject('store')
		return {
			store
		}
	},
}
</script>