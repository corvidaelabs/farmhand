<script lang="ts">
	import type { StreamData } from '$lib/server/users';
	import { format } from 'date-fns';

	interface Props {
		stream: StreamData;
		routePrefix: string;
	}
	const { stream, routePrefix }: Props = $props();
</script>

<a
	href={`${routePrefix}/${stream.id}`}
	class="my-2 flex h-36 w-60 flex-col justify-between rounded bg-surface-700 p-4 text-secondary-100 transition-colors hover:bg-surface-600"
>
	<p class="text-right text-xs font-semibold text-surface-200">
		{format(stream.start_time, 'MMMM d, yyyy')}
	</p>
	<div>
		<p class="text-2xs text-surface-300">
			Started at: {format(stream.start_time, 'h:mm a')}
		</p>
		{#if stream.end_time}
			<p class="text-2xs text-surface-300">
				Ended at: {format(stream.end_time, 'h:mm a')}
			</p>
		{:else}
			<p class="text-2xs animate-pulse text-red-500">Active</p>
		{/if}
	</div>
</a>
