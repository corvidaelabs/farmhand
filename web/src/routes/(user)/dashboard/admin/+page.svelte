<script lang="ts">
	import { enhance } from '$app/forms';
	import { goto } from '$app/navigation';
	import type { ActionResult } from '@sveltejs/kit';

	let { data } = $props();

	const handleShadowUser = () => {
		return async ({ result }: { result: ActionResult }) => {
			console.log('yoyo', result);
			if (result.type === 'success' && result.data?.token) {
				await goto(`/login?token=${result.data.token}`);
			}
		};
	};
</script>

<div class="p-4">
	<table class="min-w-full rounded bg-surface-900 shadow-md">
		<thead>
			<tr class="bg-surface-800">
				<th
					class="border-b-2 border-surface-700 px-6 py-3 text-left text-xs font-semibold uppercase tracking-wider text-surface-200"
					>Username</th
				>
				<th
					class="border-b-2 border-surface-700 px-6 py-3 text-left text-xs font-semibold uppercase tracking-wider text-surface-200"
					>Email</th
				>
				<th
					class="border-b-2 border-surface-700 px-6 py-3 text-left text-xs font-semibold uppercase tracking-wider text-surface-200"
					>Role</th
				>
				<th
					class="border-b-2 border-surface-700 px-6 py-3 text-right text-xs font-semibold uppercase tracking-wider text-surface-200"
					>Actions</th
				>
			</tr>
		</thead>
		<tbody>
			{#each data.users as user}
				<tr class="hover:bg-surface-700">
					<td class="whitespace-nowrap border-b border-surface-700 px-6 py-4 text-surface-200"
						>{user.username}</td
					>
					<td class="whitespace-nowrap border-b border-surface-700 px-6 py-4 text-surface-200"
						>{user.email}</td
					>
					<td class="whitespace-nowrap border-b border-surface-700 px-6 py-4 text-surface-200"
						>{user.role}</td
					>
					<td
						class="whitespace-nowrap border-b border-surface-700 px-6 py-4 text-right text-surface-200"
					>
						<form method="POST" action="?/shadowUser" class="inline" use:enhance={handleShadowUser}>
							<input type="hidden" name="username" value={user.username} />
							<button type="submit" class="variant-soft-primary btn">Shadow</button>
						</form>
					</td>
				</tr>
			{/each}
		</tbody>
	</table>
</div>
