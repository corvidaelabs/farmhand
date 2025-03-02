<script lang="ts">
	import type { Snippet } from 'svelte';
	import Header from '$lib/components/Header.svelte';
	import Footer from '$lib/components/Footer.svelte';

	let {
		main,
		headerActions,
		sidebar
	}: { main: Snippet; headerActions: Snippet; sidebar: Snippet } = $props();
</script>

<main class="grid min-h-screen grid-cols-12 grid-rows-main divide-x divide-y">
	<Header
		width="w-full max-w-none"
		class="dark:bg-primary-950/30 col-span-12 row-start-1 border-primary-200/20 bg-black/40 backdrop-blur-sm dark:border-primary-800/40"
		headerLink="/dashboard"
	>
		{#snippet actions()}
			{@render headerActions()}
		{/snippet}
	</Header>
	<aside
		class="col-span-2 row-start-2 min-h-max border-primary-200/20 bg-black/40 p-4 backdrop-blur-sm dark:border-primary-800/40"
	>
		{@render sidebar()}
	</aside>
	<section
		class="col-span-10 row-start-2 mx-auto min-h-full w-full flex-grow border-primary-200/20 bg-black/40 p-4 backdrop-blur-sm dark:border-primary-800/40"
	>
		{@render main()}
	</section>
	<Footer
		class="col-span-12 row-start-3 border-primary-200/20 bg-black/40 backdrop-blur-sm dark:border-primary-800/40"
	/>
</main>
