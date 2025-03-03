<script lang="ts">
	import type { Snippet } from 'svelte';
	import type { LayoutServerData } from './$types';
	import MyAccount from '$lib/components/MyAccount.svelte';
	import ThemeToggler from '$lib/components/ThemeToggler.svelte';
	import AppLayout from '$lib/components/AppLayout.svelte';
	import { UserRole } from '$lib/stores/user';

	let { children, data }: { children: Snippet; data: LayoutServerData } = $props();
	const sidebarLinks = [
		{
			link: '/dashboard/streams',
			label: 'Streams'
		}
	];
</script>

<AppLayout>
	{#snippet headerActions()}
		<nav class="mr-4 flex items-center justify-evenly space-x-4">
			<a href="/dashboard/streams">
				<span class="font-semibold">Streams</span>
			</a>
			{#if data.user.role === UserRole.ADMIN}
				<a href="/dashboard/admin">
					<span class="font-semibold">Admin</span>
				</a>
			{/if}
			<MyAccount user={data.user} />
			<ThemeToggler />
		</nav>
	{/snippet}
	{#snippet sidebar()}
		<nav>
			<ul>
				{#each sidebarLinks as link}
					<li class="font-semibold uppercase">
						<a href={link.link}>{link.label}</a>
					</li>
				{/each}
			</ul>
		</nav>
	{/snippet}
	{#snippet main()}
		{@render children()}
	{/snippet}
</AppLayout>
