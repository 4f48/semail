import { z } from 'zod';

export const register = z.object({
	username: z.string().min(1).max(64).regex(new RegExp('^[a-zA-Z0-9._-]')),
	password: z.string().min(1).max(64)
});

export const login = z.object({
	username: z.string().min(1).max(64).regex(new RegExp('^[a-zA-Z0-9._-]')),
	password: z.string().min(1).max(64)
});

export type Register = typeof register;
export type Login = typeof login;
