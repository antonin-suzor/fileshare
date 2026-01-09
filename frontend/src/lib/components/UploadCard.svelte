<script lang="ts">
    import { page } from '$app/state';
    import { axiosInstance } from '$lib/api/axios';
    import type { Upload } from '$lib/types';

    const textDisplaySizeLimit = 180;

    let { upload, onDelete }: { upload: Upload; onDelete: (upload: Upload) => void } = $props();

    async function getUploadText(): Promise<string> {
        const res = await fetch(`/content/${upload.id}/${upload.file_name}`);
        const resText = await res.text();
        const resTextLength = resText.length;
        if (resTextLength >= textDisplaySizeLimit) {
            return resText.slice(0, textDisplaySizeLimit) + `... (${resTextLength} characters)`;
        } else {
            return resText;
        }
    }

    function copyLink() {
        navigator.clipboard
            .writeText(`${page.url.origin}/content/${upload.id}/${upload.file_name}`)
            .then(() => alert('Link copied successfully !'))
            .catch(() => alert('Hmm, something went wrong... Did you give us access to your clipboard ?'));
    }

    async function deleteUpload() {
        await axiosInstance.delete(`/api/uploads/${upload.id}`);
        onDelete(upload);
    }
</script>

<div class="card w-96 bg-base-100 shadow-sm">
    <figure>
        {#if upload.content_type.startsWith('image')}
            <img
                src={`/content/${upload.id}/${upload.file_name}`}
                alt={`preview for ${upload.file_name}`}
                style="object-fit: scale-down;"
            />
        {:else if upload.content_type.startsWith('text')}
            {#await getUploadText()}
                <p>Text-based file</p>
            {:then uploadText}
                <code>{uploadText}</code>
            {/await}
        {:else}
            <p>No preview available.</p>
        {/if}
    </figure>
    <div class="card-body border-t">
        <h2 class="card-title link"><a href={`/uploads/view?id=${upload.id}`}>{upload.file_name}</a></h2>
        <div class="card-actions justify-end">
            <a
                href={`/content/${upload.id}/${upload.file_name}`}
                target="_blank"
                class="btn btn-primary"
                download={upload.file_name}>Download</a
            >
            <button class="btn btn-primary" onclick={copyLink}>Copy Link</button>
            <button class="btn btn-primary" onclick={deleteUpload}>Delete</button>
        </div>
    </div>
</div>
