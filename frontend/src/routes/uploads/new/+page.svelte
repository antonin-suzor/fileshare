<script lang="ts">
    import { goto } from '$app/navigation';
    import { startNewUpload } from '$lib/api/uploads.svelte';

    let files: FileList | null = $state(null);

    async function uploadFiles() {
        const file: File = files?.item(0)!;
        const presigned_put_url = await startNewUpload(file.name, file.type);
        await fetch(presigned_put_url, {
            method: 'PUT',
            body: file,
            headers: {
                'Content-Type': file.type,
            },
        });
        goto('/uploads');
    }
</script>

<svelte:head>
    <title>New Upload | FileShare</title>
</svelte:head>

<div class="flex justify-center-safe">
    <input type="file" class="file-input file-input-ghost" bind:files />
    <button onclick={uploadFiles} class="btn" disabled={files === null}>Upload file</button>
</div>
