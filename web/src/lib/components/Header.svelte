<script lang="ts">
	import Logo from './Logo.svelte';
	let { actions, width = 'w-full max-w-screen-xl', class: className, headerLink = '/' } = $props();
</script>

<header class="z-50 flex w-full items-center justify-center text-black dark:text-white {className}">
	<nav class="flex items-center justify-between py-4 text-sm {width}">
		<div class="justify-even-10 flex flex-nowrap items-center space-x-10">
			<a href={headerLink} class="flex flex-nowrap items-center">
				<Logo size={50} />
				<span class="font-semibold">FARMHAND</span>
			</a>
		</div>
		<aside class="flex items-center space-x-4">
			{@render actions()}
		</aside>
	</nav>
</header>
