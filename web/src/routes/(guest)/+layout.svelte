<script lang="ts">
	import type { Snippet } from 'svelte';
	import type { LayoutServerData } from './$types';
	import Header from '$lib/components/Header.svelte';
	import Footer from '$lib/components/Footer.svelte';
	import Button from '$lib/components/Button.svelte';
	import MyAccount from '$lib/components/MyAccount.svelte';
	import ThemeToggler from '$lib/components/ThemeToggler.svelte';
	import { UserRole } from '$lib/stores/user';

	let { children, data }: { children: Snippet; data: LayoutServerData } = $props();
</script>

<main class="flex min-h-screen flex-col">
	<Header class="mx-auto w-full max-w-screen-xl">
		{#snippet actions()}
			{#if data.user}
				<a href="/dashboard/streams">
					<span class="font-semibold">Streams</span>
				</a>
				{#if data.user.role === UserRole.ADMIN}
					<a href="/dashboard/admin">
						<span class="font-semibold">Admin</span>
					</a>
				{/if}
				<MyAccount user={data.user} />
			{:else}
				<Button href="/login">Log in</Button>
			{/if}
			<ThemeToggler />
		{/snippet}
	</Header>
	<section class="mx-auto flex min-h-full w-full flex-grow p-4">
		{@render children()}
	</section>
	<Footer />
</main>
