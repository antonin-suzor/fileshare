/*
DO NOT RENAME TO TS, OR CLOUDFLARE PAGES WILL CRASH ON BUILD
*/
import adapter from '@sveltejs/adapter-static';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

/** @type {import('@sveltejs/kit').Config} */
const config = {
    preprocess: vitePreprocess(),
    kit: {
        adapter: adapter(),
        prerender: {
            crawl: true,
            handleHttpError: 'fail',
            handleMissingId: 'warn',
            handleEntryGeneratorMismatch: 'fail',
            handleUnseenRoutes: 'warn',
        },
    },
};

export default config;
