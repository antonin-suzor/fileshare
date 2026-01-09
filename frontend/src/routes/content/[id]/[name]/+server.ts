import type { RequestHandler } from './$types';
import { S3Client } from 'bun';

const client = new S3Client({
    accessKeyId: process.env.S3_ACCESS_KEY_ID,
    secretAccessKey: process.env.S3_SECRET_ACCESS_KEY,
    bucket: process.env.S3_BUCKET_NAME,
    endpoint: process.env.S3_URL,
});

export const GET: RequestHandler = async ({ params }: { params: { id: string; name: string } }) => {
    const { id, name } = params;
    const s3file = client.file(`/content/${id}/${name}`);
    return new Response(s3file.stream());
};
