import { fail } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';
import { getEventsByDate, getStreamsByToken } from '$lib/server/users';

export const load: PageServerLoad = async ({ params, cookies, locals }) => {
	if (!locals.user) {
		return fail(401, {
			success: false,
			message: 'Unauthorized'
		});
	}

	const streamId = params.stream_id;
	const token = cookies.get('jwt');
	if (!token) {
		return fail(401, {
			success: false,
			message: 'Unauthorized'
		});
	}

	// Get all streams for the user with that ID
	const streams = await getStreamsByToken(token, streamId);
	// Presume the stream is the only one returned
	const stream = streams[0];
	// Get all events for that stream
	const events = await getEventsByDate(
		token,
		locals.user.username,
		stream.start_time,
		stream.end_time
	);

	return {
		stream: streams[0],
		events
	};
};
