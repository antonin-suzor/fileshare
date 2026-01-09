export interface User {
    id: string;
    created_at: string;
    updated_at: string;
    email: string;
    verified: boolean;
}

export interface Upload {
    id: string;
    user_id: string;
    created_at: string;
    updated_at: string;
    file_name: string;
    content_type: string;
    presigned_get: string;
    expires_at: string;
}
