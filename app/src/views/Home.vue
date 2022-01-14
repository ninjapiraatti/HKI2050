<template>
	<div class="container">
		<div class="row gx-4 justify-content-center">
			<div class="col-md-8">
				<div class="bigasslogo">
					<img src="/public/assets/uusilogo04_square_nobg.png" alt="">
				</div>
				<div :class="[colorScheme.card, 'p-4 mb-4']" style="--bs-bg-opacity: .8;">
					<p>The November rain falls heavy. In the city below, the delusional promises of the early 21st century never came true. Too easily, those with power took and those without paid the price.</p>
					<p>Helsinki is a very different place now. The institutions, infrastructure, and charm of the Pearl of the Baltic Sea are a thing of the past, replaced by a despairing melancholy, abused by vile corporations, corrupt officials, and ruthless factions.</p>
					<p>Yet, hope dies hard. Hope is the senseless fuel that serves you a warm bowl of hernekeitto-ramen in the steamy bazaars of Nether Helsinki. Hope is why they’re trying to rob you in the flooded high rises of Ruoholahti. And ultimately, hope is what will claim you in surviving HKI2050.</p>
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