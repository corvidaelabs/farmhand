import { env } from '$env/dynamic/private';
import type { User } from '$lib/stores/user';

export enum UserError {
	INVALID_TOKEN = 'INVALID_TOKEN',
	UNKNOWN = 'UNKNOWN'
}

export const getTokenIdentity = async (token: string): Promise<User | null> => {
	try {
		const headers = {
			Authorization: `Bearer ${token}`,
			'Content-Type': 'application/json'
		};

		const response = await fetch(`${env.API_URL}/user/me`, {
			method: 'GET',
			headers
		});

		if (response.ok) {
			const userData: User = await response.json();
			return userData;
		} else {
			console.error('Response status:', response.status);
			console.error('Response headers:', response.headers);
			const errorText = await response.text();
			console.error('Error response:', errorText);
			throw UserError.INVALID_TOKEN;
		}
	} catch (e) {
		console.error('Error in getTokenIdentity:', e);
		if (e === UserError.INVALID_TOKEN) {
			throw e;
		}
		throw UserError.UNKNOWN;
	}
};

export const getUserByEmail = async (email: string, token: string): Promise<User | null> => {
	try {
		const response = await fetch(`${env.API_URL}/user?email=${encodeURIComponent(email)}`, {
			headers: {
				Authorization: `Bearer ${token}`
			}
		});

		if (response.ok) {
			const userData: User = await response.json();
			return userData;
		} else if (response.status === 404) {
			return null;
		} else {
			throw UserError.INVALID_TOKEN;
		}
	} catch (e) {
		if (e === UserError.INVALID_TOKEN) {
			throw e;
		}
		throw UserError.UNKNOWN;
	}
};

export type StreamData = {
	id: string;
	start_time: string;
	end_time: string | null;
	event_log_url: string | null;
	video_url: string | null;
	created_at: string;
	updated_at: string;
};

export interface StreamResponse {
	streams: StreamData[];
}

export const getStreamsByToken = async (
	token: string,
	streamID?: string
): Promise<StreamData[]> => {
	try {
		const url = new URL(`${env.API_URL}/user/streams`);
		if (streamID) {
			url.searchParams.append('stream_id', streamID);
		}

		const response = await fetch(url, {
			headers: {
				Authorization: `Bearer ${token}`
			}
		});

		if (response.ok) {
			const data = (await response.json()) as StreamResponse;
			return data.streams;
		} else {
			throw UserError.INVALID_TOKEN;
		}
	} catch (e) {
		if (e === UserError.INVALID_TOKEN) {
			throw e;
		}
		throw UserError.UNKNOWN;
	}
};

export const getEventsByDate = async (
	token: string,
	username: string,
	startTime: string,
	endTime: string | null
): Promise<any[]> => {
	try {
		const url = new URL(`${env.API_URL}/user/events`);
		url.searchParams.append('username', username);
		url.searchParams.append('start_time', startTime);
		if (endTime) {
			url.searchParams.append('end_time', endTime);
		}

		const response = await fetch(url, {
			headers: {
				Authorization: `Bearer ${token}`
			}
		});

		if (response.ok) {
			const data = (await response.json()) as { events: any[] };
			return data.events;
		} else {
			throw UserError.INVALID_TOKEN;
		}
	} catch (e) {
		if (e === UserError.INVALID_TOKEN) {
			throw e;
		}
		throw UserError.UNKNOWN;
	}
};

// Get all users
export const getAllUsers = async (token: string): Promise<User[]> => {
	try {
		const url = new URL(`${env.API_URL}/user`);
		const response = await fetch(url, {
			headers: {
				Authorization: `Bearer ${token}`
			}
		});

		if (response.ok) {
			const json = await response.json();
			const data = json as { users: User[] };
			return data.users;
		} else {
			throw UserError.INVALID_TOKEN;
		}
	} catch (e) {
		if (e === UserError.INVALID_TOKEN) {
			throw e;
		}
		throw UserError.UNKNOWN;
	}
};

// Shadow a user
export const getShadowUserToken = async (token: string, username: string): Promise<string> => {
	try {
		const response = await fetch(`${env.API_URL}/auth/shadow`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json',
				Authorization: `Bearer ${token}`
			},
			body: JSON.stringify({ username })
		});

		if (!response.ok) {
			throw UserError.INVALID_TOKEN;
		}

		const data = (await response.json()) as { token: string };
		return data.token;
	} catch (e) {
		if (e === UserError.INVALID_TOKEN) {
			throw e;
		}
		throw UserError.UNKNOWN;
	}
};
