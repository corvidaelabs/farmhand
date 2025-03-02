import { fail, redirect } from '@sveltejs/kit';
import type { LayoutServerLoad } from './$types';
import { getStreamsByToken } from '$lib/server/users';

export const load: LayoutServerLoad = async ({ locals, cookies }) => {
	if (!locals.user) {
		throw redirect(307, '/login');
	}

	const token = cookies.get('jwt');
	if (!token) {
		return fail(401, {
			success: false,
			message: 'Unauthorized'
		});
	}

	const streams = await getStreamsByToken(token);
	return {
		user: locals.user,
		streams
	};
};
