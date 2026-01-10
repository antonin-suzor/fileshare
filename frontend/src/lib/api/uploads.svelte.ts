import type { Upload } from '../types';
import { axiosInstance } from './axios';

let myUploads: Upload[] = $state([]);
export function lazyGetUserUploads() {
    return myUploads;
}
export function lazySetUserUploads(newUserUploads: Upload[]) {
    myUploads = newUserUploads;
}
export async function lazyRefreshUserUploads() {
    try {
        const res = await axiosInstance.get('/api/uploads/mine');
        myUploads = res.data as Upload[];
    } catch (e) {
        console.error(e);
    }
}

export async function startNewUpload(file_name: string, content_type: string): Promise<string> {
    try {
        const res = await axiosInstance.post('/api/uploads/start', {
            file_name,
            content_type,
            expires_at: new Date(Date.now() + 1000 * 60 * 60 * 24),
        });
        return res.data.url;
    } catch (e) {
        console.error(e);
        throw e;
    }
}

export async function getUpload(id: string): Promise<Upload | null> {
    try {
        const res = await axiosInstance.get(`/api/uploads/${id}`);
        return res.data as Upload;
    } catch (e) {
        console.error(e);
        return null;
    }
}
