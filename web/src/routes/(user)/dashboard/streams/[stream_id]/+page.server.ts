import { fail } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';
import { getStreamsByToken } from '$lib/server/users';

export const load: PageServerLoad = async ({ params, cookies }) => {
	const streamId = params.stream_id;
	const token = cookies.get('jwt');
	if (!token) {
		return fail(401, {
			success: false,
			message: 'Unauthorized'
		});
	}

	const streams = await getStreamsByToken(token, streamId);
	return {
		stream: streams[0]
	};
};
