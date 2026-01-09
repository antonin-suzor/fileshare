import axios from 'axios';
import { getToken } from './auth.svelte';

export const axiosInstance = axios.create({
    allowAbsoluteUrls: true,
    baseURL: import.meta.env.VITE_API_HOST,
});

axiosInstance.interceptors.request.use((config) => {
    config.headers.Authorization = `Bearer ${getToken()}`;
    return config;
});
