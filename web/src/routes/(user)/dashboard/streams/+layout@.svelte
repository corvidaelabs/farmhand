<script lang="ts">
	import type { Snippet } from 'svelte';
	import type { LayoutServerData } from './$types';
	import MyAccount from '$lib/components/MyAccount.svelte';
	import ThemeToggler from '$lib/components/ThemeToggler.svelte';
	import AppLayout from '$lib/components/AppLayout.svelte';
	import Alert from '$lib/components/Alert.svelte';
	import StreamCard from '$lib/components/StreamCard.svelte';

	let { children, data }: { children: Snippet; data: LayoutServerData } = $props();
	const streams = data.streams || [];
</script>

<AppLayout>
	{#snippet headerActions()}
		<nav class="mr-4 flex items-center justify-evenly space-x-4">
			<a href="/dashboard">
				<span class="font-semibold">Dashboard</span>
			</a>
			<MyAccount user={data.user} />
			<ThemeToggler />
		</nav>
	{/snippet}
	{#snippet sidebar()}
		{#each streams as stream}
			<StreamCard {stream} routePrefix="/dashboard/streams" />
		{/each}
	{/snippet}
	{#snippet main()}
		<Alert message="This page is still under construction" type="warning" class="mb-4" />
		{@render children()}
	{/snippet}
</AppLayout>
