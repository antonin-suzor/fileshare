import { goto } from '$app/navigation';
import { page } from '$app/state';
import type { User } from '../types';
import { axiosInstance } from './axios';

const localStorageTokenKey = 'FILESHARE_AUTH_TOKEN'

let activeToken: string = $state('');
let currentUser: User | undefined = $state();

let loggedIn: boolean = $derived(activeToken !== '');
let verified: boolean = $derived(currentUser ? currentUser.verified : false);

export const isLoggedIn = () => loggedIn;
export const isVerified = () => verified;

export async function getCurrentUser(): Promise<User | null> {
    if (currentUser) {
        return currentUser;
    } else if (activeToken) {
        try {
            const res = await axiosInstance.get('/api/users/me');
            currentUser = res.data;
            return currentUser as User;
        } catch (e) {
            console.error(e);
            return null;
        }
    } else {
        return null;
    }
}

export async function initAuth() {
    try {
        const localStorageToken = localStorage.getItem(localStorageTokenKey);
        if (localStorageToken !== null && localStorageToken !== '') {
            activeToken = localStorageToken;
        }
    } finally {
        if (loggedIn) {
            const res = await axiosInstance.get('/api/users/me');
            currentUser = res.data as User;
        }
    }
}

export async function requireLoggedIn() {
    await initAuth();
    if (!isLoggedIn()) {
        await goto(`/account/login?redirect=${page.route.id}`);
    }
}

export async function requireVerifiedUser() {
    await initAuth();
    if (!isVerified()) {
        await goto(`/account/verify-email?redirect=${page.route.id}`);
    }
}

export function getToken(): string {
    try {
        const localStorageToken = localStorage.getItem(localStorageTokenKey);
        if (localStorageToken !== null && localStorageToken !== '') {
            activeToken = localStorageToken;
        }
    } finally {
        return activeToken;
    }
}

export function setToken(token: string) {
    activeToken = token;
    try {
        localStorage.setItem(localStorageTokenKey, token);
    } catch {
        console.warn(
            'Looks like local storage is deactivated. You can still use the website, but will be logged out if you change/close the tab, and some features might not work as expected.'
        );
    }
}

export function clearToken() {
    activeToken = '';
    try {
        localStorage.removeItem(localStorageTokenKey);
    } catch {}
}

export async function signup(email: string, password: string): Promise<boolean> {
    try {
        const res = await fetch(`${import.meta.env.VITE_API_HOST}/api/users/signup`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({ email, password }),
        });
        const resBody = await res.json();
        if (!res.ok) {
            alert(resBody.message);
            return false;
        }
        setToken(resBody.token);
        currentUser = resBody.user;
        return true;
    } catch (e) {
        console.error(e);
        return false;
    }
}

export async function login(email: string, password: string): Promise<boolean> {
    try {
        const res = await fetch(`${import.meta.env.VITE_API_HOST}/api/users/login`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({ email, password }),
        });
        const resBody = await res.json();
        if (!res.ok) {
            alert(resBody.message);
            return false;
        }
        setToken(resBody.token);
        currentUser = resBody.user;
        return true;
    } catch (e) {
        console.error(e);
        return false;
    }
}

export function logout(): boolean {
    setToken('');
    currentUser = undefined;
    return true;
}

export async function verify(verificationId: string): Promise<boolean> {
    try {
        const res = await axiosInstance.post(`/api/users/verify/${verificationId}`);
        setToken(res.data.token);
        currentUser = res.data.user as User;
        return true;
    } catch (e) {
        console.error(e);
        return false;
    }
}

export async function sendVerification(): Promise<boolean> {
    try {
        const res = await axiosInstance.post(`/api/users/me/send-verification`);
        return true;
    } catch (e) {
        console.error(e);
        return false;
    }
}
