/**
 * Request handler for the CloudFront Distribution
 * @param any event incoming AWS CloudFront object with request information
 * @returns Modified request
 */
function handler(event) {
    let req = event.request;

    // don't modify api requests
    if (req.uri.startsWith("/api")) {
        return req;
    }

    // don't modify requests for explicit files
    if (req.uri.split("/").pop().includes(".")) {
        return req;
    }

    // rewrite all other requests to Svelte MPA
    if (req.uri.endsWith("/")) {
        req.uri += "index.html";
    } else {
        req.uri += "/index.html";
    }

    return req;
}
