<script lang="ts">
	import { format } from 'date-fns';

	const { data } = $props();
</script>

<section class="flex flex-col space-y-4">
	<div class="rounded-lg p-4 shadow-sm">
		<h2 class="mb-4 text-xl font-semibold">Stream Details</h2>

		<div class="space-y-2">
			<p class="text-surface-300">Stream ID: <span class="font-medium">{data.stream?.id}</span></p>

			{#if data.stream?.start_time}
				<p class="text-surface-300">
					Started: <span class="font-medium"
						>{format(data.stream.start_time, 'MMMM do, yyyy h:mm a')}</span
					>
				</p>
			{/if}

			{#if data.stream?.end_time}
				<p class="text-surface-300">
					Ended: <span class="font-medium"
						>{format(data.stream.end_time, 'MMMM do, yyyy h:mm a')}</span
					>
				</p>
			{/if}

			{#if data.events}
				<p class="text-surface-300">
					Total Events: <span class="font-medium">{data.events.length}</span>
				</p>
			{/if}
		</div>
	</div>
</section>
