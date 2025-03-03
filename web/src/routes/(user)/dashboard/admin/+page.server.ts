import { fail, redirect } from '@sveltejs/kit';
import type { Actions, PageServerLoad } from './$types';
import { getAllUsers, getShadowUserToken } from '$lib/server/users';

export const load: PageServerLoad = async ({ locals, cookies }) => {
	if (!locals.user) {
		throw redirect(303, '/login');
	}

	const token = cookies.get('jwt');
	if (!token) {
		throw redirect(303, '/login');
	}

	const allUsers = await getAllUsers(token);
	return {
		users: allUsers
	};
};

export const actions = {
	shadowUser: async ({ request, fetch, locals, cookies }) => {
		const formData = await request.formData();
		const targetUsername = formData.get('username');
		const token = cookies.get('jwt');
		if (!targetUsername || !token) {
			return fail(400, { error: 'Bad Credentials' });
		}

		const shadowUserToken = await getShadowUserToken(token, targetUsername.toString());
		return {
			token: shadowUserToken
		};
	}
} satisfies Actions;
