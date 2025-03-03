<script lang="ts">
	import type { Snippet } from 'svelte';
	import type { LayoutServerData } from './$types';
	import MyAccount from '$lib/components/MyAccount.svelte';
	import ThemeToggler from '$lib/components/ThemeToggler.svelte';
	import AppLayout from '$lib/components/AppLayout.svelte';
	import Alert from '$lib/components/Alert.svelte';
	import StreamCard from '$lib/components/StreamCard.svelte';
	import { UserRole } from '$lib/stores/user';

	let { children, data }: { children: Snippet; data: LayoutServerData } = $props();
	const streams = data.streams || [];
</script>

<AppLayout>
	{#snippet headerActions()}
		<nav class="mr-4 flex items-center justify-evenly space-x-4">
			<a href="/dashboard/streams">
				<span class="font-semibold">Streams</span>
			</a>
			{#if data.user?.role === UserRole.ADMIN}
				<a href="/dashboard/admin">
					<span class="font-semibold">Admin</span>
				</a>
			{/if}
			<MyAccount user={data.user} />
			<ThemeToggler />
		</nav>
	{/snippet}
	{#snippet sidebar()}
		{#each streams as stream}
			<StreamCard
				{stream}
				routePrefix="/dashboard/streams"
				isActive={stream.id === data.activeStreamID}
			/>
		{/each}
	{/snippet}
	{#snippet main()}
		<Alert message="This page is still under construction" type="warning" class="mb-4" />
		{@render children()}
	{/snippet}
</AppLayout>
