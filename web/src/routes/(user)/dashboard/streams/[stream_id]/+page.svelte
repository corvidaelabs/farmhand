<script lang="ts">
	import { format } from 'date-fns';

	const { data } = $props();
	console.log(data);
</script>

<section class="flex flex-col space-y-4">
	<p>You're looking at the page for {data.stream?.id}</p>
	{#if data.stream?.start_time}
		<p>Stream started at {format(data.stream.start_time, 'yyyy-MM-dd HH:mm:ss')}</p>
	{/if}
	{#if data.stream?.end_time}
		<p>Stream ended at {format(data.stream.end_time, 'yyyy-MM-dd HH:mm:ss')}</p>
	{/if}
	{#if data.events}
		Total Events: {data.events.length}
	{/if}
</section>
